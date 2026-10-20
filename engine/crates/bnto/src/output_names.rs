// Output names for one run, kept apart on every filesystem.
//
// Two result files whose names match, or differ only in case, would land on
// one file on Windows and macOS (an exact match on Linux too), so the later
// one is written as `stem_1.ext`, `stem_2.ext`…, the suffix `file-copy`'s
// `rename` conflict mode uses. Case is folded on every OS so a recipe writes
// the same files everywhere. Names are kept apart from each other only, not
// from files already in the output folder: a rerun replaces its own output.

use std::collections::HashSet;

/// A result file written under a different name than the pipeline gave it.
#[derive(Debug, Clone, PartialEq)]
pub struct Renamed {
    pub from: String,
    pub to: String,
}

/// The names claimed so far in one run's output folder.
#[derive(Debug, Default)]
pub struct OutputNames {
    taken: HashSet<String>,
}

impl OutputNames {
    /// The name to write `name` under: itself when free, else the first free
    /// suffixed form. Either way it's taken from then on.
    pub fn claim(&mut self, name: &str) -> String {
        let mut chosen = name.to_string();
        let mut n = 0;
        while self.taken.contains(&fold(&chosen)) {
            n += 1;
            chosen = suffixed(name, n);
        }
        self.taken.insert(fold(&chosen));
        chosen
    }
}

/// Each name's write name, in order, and the ones that changed.
pub fn plan<'a>(names: impl IntoIterator<Item = &'a str>) -> (Vec<String>, Vec<Renamed>) {
    let mut claimed = OutputNames::default();
    let mut chosen = Vec::new();
    let mut renamed = Vec::new();
    for name in names {
        let to = claimed.claim(name);
        if to != name {
            renamed.push(Renamed {
                from: name.to_string(),
                to: to.clone(),
            });
        }
        chosen.push(to);
    }
    (chosen, renamed)
}

/// The key two names share when they'd be one file on some OS.
fn fold(name: &str) -> String {
    name.replace('\\', "/").to_lowercase()
}

/// `name` with `_n` before its extension: `sub/logo.svg` → `sub/logo_1.svg`.
/// A leading dot starts the name, not an extension (`.env` → `.env_1`).
fn suffixed(name: &str, n: usize) -> String {
    let leaf_start = name.rfind(['/', '\\']).map_or(0, |i| i + 1);
    let ext_start = name[leaf_start..]
        .rfind('.')
        .filter(|&i| i > 0)
        .map_or(name.len(), |i| leaf_start + i);
    format!("{}_{n}{}", &name[..ext_start], &name[ext_start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(names: &[&str]) -> Vec<String> {
        plan(names.iter().copied()).0
    }

    #[test]
    fn test_distinct_names_keep_their_names() {
        assert_eq!(claims(&["a.svg", "b.svg"]), ["a.svg", "b.svg"]);
    }

    #[test]
    fn test_names_differing_only_in_case_are_kept_apart() {
        // Guards: `Logo.svg` and `logo.svg` were one file on Windows, and the
        // second write silently replaced the first.
        assert_eq!(
            claims(&["Logo.svg", "logo.svg"]),
            ["Logo.svg", "logo_1.svg"]
        );
    }

    #[test]
    fn test_an_exact_repeat_is_kept_apart() {
        assert_eq!(
            claims(&["logo.svg", "logo.svg", "LOGO.svg"]),
            ["logo.svg", "logo_1.svg", "LOGO_2.svg"]
        );
    }

    #[test]
    fn test_the_suffix_goes_on_the_file_name_not_its_folder() {
        assert_eq!(
            claims(&["Icons/a.svg", "icons/A.svg"]),
            ["Icons/a.svg", "icons/A_1.svg"]
        );
    }

    #[test]
    fn test_names_in_one_folder_by_either_separator_clash() {
        assert_eq!(
            claims(&["sub/a.svg", "sub\\a.svg"]),
            ["sub/a.svg", "sub\\a_1.svg"]
        );
    }

    #[test]
    fn test_a_name_without_an_extension_takes_the_suffix_at_the_end() {
        assert_eq!(claims(&["README", "readme"]), ["README", "readme_1"]);
        assert_eq!(claims(&[".env", ".ENV"]), [".env", ".ENV_1"]);
    }

    #[test]
    fn test_a_suffixed_name_never_takes_a_later_files_name_from_it() {
        // `a_1.svg` is claimed by the case twin, so the real `a_1.svg` moves on.
        assert_eq!(
            claims(&["a.svg", "A.svg", "a_1.svg"]),
            ["a.svg", "A_1.svg", "a_1_1.svg"]
        );
    }

    #[test]
    fn test_plan_lists_only_the_names_that_changed() {
        let (_, renamed) = plan(["x.svg", "Logo.svg", "logo.svg"]);
        assert_eq!(
            renamed,
            [Renamed {
                from: "logo.svg".into(),
                to: "logo_1.svg".into()
            }]
        );
    }

    #[test]
    fn test_one_registry_keeps_names_apart_across_claims() {
        // The progressive writer claims one iteration at a time; the names it
        // picks must match the final write's plan over the whole result.
        let mut names = OutputNames::default();
        let progressive = [names.claim("Logo.svg"), names.claim("logo.svg")];
        assert_eq!(progressive.to_vec(), claims(&["Logo.svg", "logo.svg"]));
    }
}
