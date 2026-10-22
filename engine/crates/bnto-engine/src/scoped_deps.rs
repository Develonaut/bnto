// Which dependencies a recipe needs on each OS. A node's own `requires`, and
// its processor's, count only on the OSes the node runs on
// (`os_support::runs_on`); the recipe's root `requires` count everywhere.
// See decisions/conditional-steps.md, "Dependencies in scope (PR 8)".

use std::collections::HashSet;

use bnto_core::os_support::{describe_os, runs_on};
use bnto_core::{Dependency, HostPlatform, NodeRegistry, PipelineDefinition, PipelineNode};

/// A dependency and the OSes that need it, in `HostPlatform::ALL` order.
#[derive(Debug, Clone, PartialEq)]
pub struct ScopedDependency {
    pub dependency: Dependency,
    pub os: Vec<HostPlatform>,
}

impl ScopedDependency {
    /// Whether `host` needs it. A host bnto has no name for (a BSD) can't be
    /// ruled out, so it needs everything.
    pub fn needed_on(&self, host: Option<HostPlatform>) -> bool {
        host.is_none_or(|os| self.os.contains(&os))
    }

    /// Whether some OS doesn't need it.
    pub fn is_scoped(&self) -> bool {
        self.os.len() < HostPlatform::ALL.len()
    }

    /// "macOS only" for a scoped dependency, `None` when every OS needs it.
    pub fn os_note(&self) -> Option<String> {
        self.is_scoped()
            .then(|| format!("{} only", describe_os(&self.os)))
    }

    /// How to install it on `host`, or, when `host` doesn't need it, on the
    /// first OS that does (a hint for an OS that never runs it helps no one).
    pub fn install_hint(&self, host: Option<HostPlatform>) -> &str {
        let os = if self.needed_on(host) {
            host
        } else {
            self.os.first().copied()
        };
        self.dependency.install_hint_for(os)
    }
}

/// Dependencies deduplicated by binary; the first one listed wins.
#[derive(Default)]
pub(crate) struct DepList {
    seen: HashSet<String>,
    deps: Vec<Dependency>,
}

impl DepList {
    pub(crate) fn extend<'a>(&mut self, deps: impl IntoIterator<Item = &'a Dependency>) {
        for dep in deps {
            if self.seen.insert(dep.binary.clone()) {
                self.deps.push(dep.clone());
            }
        }
    }

    pub(crate) fn into_vec(self) -> Vec<Dependency> {
        self.deps
    }
}

/// Add what `def` needs on `os`: its root `requires`, then, for each node
/// that runs on `os`, the node's own `requires` and its processor's, in
/// walk order. A node ruled out on `os` takes its whole subtree with it.
pub(crate) fn add_pipeline_deps(
    list: &mut DepList,
    def: &PipelineDefinition,
    registry: &NodeRegistry,
    os: HostPlatform,
) {
    list.extend(&def.requires);
    add_node_deps(list, &def.nodes, registry, os);
}

fn add_node_deps(
    list: &mut DepList,
    nodes: &[PipelineNode],
    registry: &NodeRegistry,
    os: HostPlatform,
) {
    let no_params = serde_json::Map::new();
    for node in nodes.iter().filter(|node| runs_on(node, os)) {
        list.extend(&node.requires);
        if let Some(processor) = registry.resolve(&node.node_type, &no_params) {
            list.extend(&processor.metadata().requires);
        }
        if let Some(children) = &node.children {
            add_node_deps(list, children, registry, os);
        }
    }
}

/// Merge what each OS needs (`needs(os)`) into one entry per distinct
/// dependency, tagged with the OSes that need it. Listed in macOS order,
/// then what only later OSes need.
pub(crate) fn scope(needs: impl Fn(HostPlatform) -> Vec<Dependency>) -> Vec<ScopedDependency> {
    let mut scoped: Vec<ScopedDependency> = Vec::new();
    for os in HostPlatform::ALL {
        for dep in needs(os) {
            match scoped.iter_mut().find(|s| s.dependency == dep) {
                Some(entry) => entry.os.push(os),
                None => scoped.push(ScopedDependency {
                    dependency: dep,
                    os: vec![os],
                }),
            }
        }
    }
    scoped
}

/// Split into what `host` needs, one per binary, and what only other OSes
/// need (a binary `host` needs isn't repeated there).
pub fn split_by_host(
    scoped: Vec<ScopedDependency>,
    host: Option<HostPlatform>,
) -> (Vec<ScopedDependency>, Vec<ScopedDependency>) {
    let (here, elsewhere): (Vec<_>, Vec<_>) = scoped.into_iter().partition(|s| s.needed_on(host));
    let mut seen = HashSet::new();
    let here: Vec<_> = here
        .into_iter()
        .filter(|s| seen.insert(s.dependency.binary.clone()))
        .collect();
    let elsewhere = elsewhere
        .into_iter()
        .filter(|s| !seen.contains(&s.dependency.binary))
        .collect();
    (here, elsewhere)
}

/// The dependencies `host` needs, one per binary.
pub(crate) fn needed_on(
    scoped: Vec<ScopedDependency>,
    host: Option<HostPlatform>,
) -> Vec<Dependency> {
    let (here, _) = split_by_host(scoped, host);
    here.into_iter().map(|s| s.dependency).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::NodeProcessor;
    use bnto_core::context::ProcessContext;
    use bnto_core::errors::BntoError;
    use bnto_core::metadata::{InputCardinality, NodeCategory, NodeMetadata};
    use bnto_core::processor::{NodeInput, NodeOutput};
    use bnto_core::progress::ProgressReporter;
    use serde_json::{Value, json};

    use HostPlatform::{Linux, MacOs, Windows};

    /// A step whose processor needs `ffmpeg`.
    struct Transcode;

    impl NodeProcessor for Transcode {
        fn name(&self) -> &str {
            "transcode"
        }

        fn process(
            &self,
            _input: NodeInput,
            _progress: &ProgressReporter,
            _ctx: &dyn ProcessContext,
        ) -> Result<NodeOutput, BntoError> {
            unreachable!("never run")
        }

        fn metadata(&self) -> NodeMetadata {
            NodeMetadata {
                node_type: "transcode".into(),
                name: "Transcode".into(),
                description: String::new(),
                category: NodeCategory::Data,
                accepts: vec![],
                platforms: vec!["cli".into()],
                parameters: vec![],
                input_cardinality: InputCardinality::PerFile,
                requires: vec![dep("ffmpeg")],
            }
        }
    }

    fn registry() -> NodeRegistry {
        let mut registry = NodeRegistry::new();
        registry.register("transcode", Box::new(Transcode));
        registry
    }

    fn dep(binary: &str) -> Dependency {
        serde_json::from_value(json!({ "binary": binary })).unwrap()
    }

    fn def(json: Value) -> PipelineDefinition {
        serde_json::from_value(json).unwrap()
    }

    fn platform(expected: &str) -> Value {
        json!({ "value": "{{ctx.platform}}", "op": "equals", "expected": expected })
    }

    fn os_group(id: &str, expected: &str, requires: &[&str], steps: Value) -> Value {
        json!({ "id": id, "type": "group", "when": platform(expected),
            "requires": requires.iter().map(|b| json!({ "binary": b })).collect::<Vec<_>>(),
            "nodes": steps })
    }

    fn step(node_type: &str) -> Value {
        json!({ "id": format!("{node_type}-step"), "type": node_type })
    }

    fn needs(def: &PipelineDefinition, os: HostPlatform) -> Vec<String> {
        let mut list = DepList::default();
        add_pipeline_deps(&mut list, def, &registry(), os);
        list.into_vec().into_iter().map(|d| d.binary).collect()
    }

    fn scoped(def: &PipelineDefinition) -> Vec<(String, Vec<HostPlatform>)> {
        scope(|os| {
            let mut list = DepList::default();
            add_pipeline_deps(&mut list, def, &registry(), os);
            list.into_vec()
        })
        .into_iter()
        .map(|s| (s.dependency.binary, s.os))
        .collect()
    }

    // --- add_pipeline_deps ---

    #[test]
    fn a_macos_groups_dependency_is_needed_only_on_macos() {
        let def = def(json!({ "nodes": [
            os_group("mac", "macos", &["caffeinate", "osascript"], json!([step("shell-run")]))
        ] }));
        assert_eq!(needs(&def, MacOs), ["caffeinate", "osascript"]);
        assert!(needs(&def, Linux).is_empty());
        assert!(needs(&def, Windows).is_empty());
    }

    #[test]
    fn root_requires_are_needed_everywhere() {
        let def = def(json!({ "requires": [{ "binary": "yt-dlp" }],
            "nodes": [os_group("mac", "macos", &[], json!([step("shell-run")]))] }));
        for os in HostPlatform::ALL {
            assert_eq!(needs(&def, os), ["yt-dlp"], "{os:?}");
        }
    }

    #[test]
    fn an_unguarded_steps_requires_are_needed_everywhere() {
        let def = def(json!({ "nodes": [
            { "id": "s", "type": "shell-run", "requires": [{ "binary": "jq" }] }
        ] }));
        for os in HostPlatform::ALL {
            assert_eq!(needs(&def, os), ["jq"], "{os:?}");
        }
    }

    #[test]
    fn a_processors_dependency_inside_a_guarded_group_is_scoped_too() {
        let def =
            def(json!({ "nodes": [os_group("lin", "linux", &[], json!([step("transcode")]))] }));
        assert_eq!(needs(&def, Linux), ["ffmpeg"]);
        assert!(needs(&def, MacOs).is_empty());
    }

    #[test]
    fn guards_nest_so_an_outer_guard_rules_out_an_inner_groups_requires() {
        let inner = os_group(
            "lin",
            "linux",
            &["systemd-inhibit"],
            json!([step("shell-run")]),
        );
        let outer = json!({ "id": "not-win", "type": "group", "nodes": [inner],
            "when": { "value": "{{ctx.platform}}", "op": "not-equals", "expected": "windows" } });
        let def = def(json!({ "nodes": [outer] }));
        assert_eq!(needs(&def, Linux), ["systemd-inhibit"]);
        assert!(needs(&def, MacOs).is_empty());
        assert!(needs(&def, Windows).is_empty());
    }

    #[test]
    fn a_guard_on_a_field_rules_no_os_out() {
        let def = def(json!({ "nodes": [{ "id": "g", "type": "group",
            "when": { "value": "{{fields.mode}}", "op": "equals", "expected": "fast" },
            "requires": [{ "binary": "jq" }], "nodes": [step("shell-run")] }] }));
        for os in HostPlatform::ALL {
            assert_eq!(needs(&def, os), ["jq"], "{os:?}");
        }
    }

    #[test]
    fn a_binary_listed_twice_keeps_the_first_entry() {
        let def = def(
            json!({ "requires": [{ "binary": "ffmpeg", "version": ">=6" }],
            "nodes": [step("transcode")] }),
        );
        let mut list = DepList::default();
        add_pipeline_deps(&mut list, &def, &registry(), Linux);
        let deps = list.into_vec();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, ">=6");
    }

    // --- scope / needed_on ---

    #[test]
    fn scope_tags_each_dependency_with_the_oses_that_need_it() {
        let def = def(json!({ "requires": [{ "binary": "git" }], "nodes": [
            os_group("mac", "macos", &["caffeinate"], json!([step("shell-run")])),
            os_group("lin", "linux", &["systemd-inhibit"], json!([step("shell-run")])),
        ] }));
        assert_eq!(
            scoped(&def),
            [
                ("git".to_string(), vec![MacOs, Linux, Windows]),
                ("caffeinate".to_string(), vec![MacOs]),
                ("systemd-inhibit".to_string(), vec![Linux]),
            ]
        );
    }

    #[test]
    fn one_binary_declared_differently_per_os_stays_two_entries() {
        let mac = json!({ "id": "mac", "type": "group", "when": platform("macos"),
            "requires": [{ "binary": "ffmpeg", "installHint": "brew install ffmpeg" }],
            "nodes": [step("shell-run")] });
        let lin = json!({ "id": "lin", "type": "group", "when": platform("linux"),
            "requires": [{ "binary": "ffmpeg", "installHint": "apt install ffmpeg" }],
            "nodes": [step("shell-run")] });
        let entries = scope(|os| {
            let mut list = DepList::default();
            add_pipeline_deps(
                &mut list,
                &def(json!({ "nodes": [mac.clone(), lin.clone()] })),
                &registry(),
                os,
            );
            list.into_vec()
        });
        let linux = needed_on(entries.clone(), Some(Linux));
        assert_eq!(linux.len(), 1);
        assert_eq!(linux[0].install_hint, "apt install ffmpeg");
        assert!(needed_on(entries, Some(Windows)).is_empty());
    }

    #[test]
    fn an_unnamed_host_needs_everything_once() {
        let def = def(json!({ "nodes": [
            os_group("mac", "macos", &["caffeinate"], json!([step("shell-run")])),
            os_group("lin", "linux", &["caffeinate"], json!([step("shell-run")])),
        ] }));
        let entries = scope(|os| {
            let mut list = DepList::default();
            add_pipeline_deps(&mut list, &def, &registry(), os);
            list.into_vec()
        });
        let binaries: Vec<String> = needed_on(entries, None)
            .into_iter()
            .map(|d| d.binary)
            .collect();
        assert_eq!(binaries, ["caffeinate"]);
    }

    #[test]
    fn split_by_host_keeps_one_entry_per_binary_on_each_side() {
        let mac = json!({ "id": "mac", "type": "group", "when": platform("macos"),
            "requires": [{ "binary": "ffmpeg", "installHint": "brew install ffmpeg" }, { "binary": "caffeinate" }],
            "nodes": [step("shell-run")] });
        let lin = json!({ "id": "lin", "type": "group", "when": platform("linux"),
            "requires": [{ "binary": "ffmpeg", "installHint": "apt install ffmpeg" }],
            "nodes": [step("shell-run")] });
        let def = def(json!({ "nodes": [mac, lin] }));
        let entries = scope(|os| {
            let mut list = DepList::default();
            add_pipeline_deps(&mut list, &def, &registry(), os);
            list.into_vec()
        });
        let (here, elsewhere) = split_by_host(entries, Some(Linux));
        let names = |v: &[ScopedDependency]| {
            v.iter()
                .map(|s| s.dependency.binary.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&here), ["ffmpeg"]);
        assert_eq!(names(&elsewhere), ["caffeinate"]);
    }

    #[test]
    fn os_note_names_the_oses_of_a_scoped_dependency() {
        let both = ScopedDependency {
            dependency: dep("x"),
            os: vec![MacOs, Linux],
        };
        let everywhere = ScopedDependency {
            dependency: dep("git"),
            os: HostPlatform::ALL.to_vec(),
        };
        assert_eq!(both.os_note().as_deref(), Some("macOS and Linux only"));
        assert_eq!(everywhere.os_note(), None);
    }

    #[test]
    fn install_hint_falls_back_to_an_os_that_needs_it() {
        let caffeinate: Dependency = serde_json::from_value(json!({ "binary": "caffeinate",
            "installHints": { "macos": "built into macOS" } }))
        .unwrap();
        let mac = ScopedDependency {
            dependency: caffeinate,
            os: vec![MacOs],
        };
        assert_eq!(mac.install_hint(Some(Linux)), "built into macOS");
        assert_eq!(mac.install_hint(Some(MacOs)), "built into macOS");
    }

    #[test]
    fn is_scoped_holds_when_some_os_does_not_need_it() {
        let everywhere = ScopedDependency {
            dependency: dep("git"),
            os: HostPlatform::ALL.to_vec(),
        };
        let mac = ScopedDependency {
            dependency: dep("caffeinate"),
            os: vec![MacOs],
        };
        assert!(!everywhere.is_scoped());
        assert!(mac.is_scoped());
        assert!(mac.needed_on(Some(MacOs)));
        assert!(!mac.needed_on(Some(Linux)));
        assert!(mac.needed_on(None));
    }
}
