// What a message-mode run reports: the text its steps produced (keep-awake's
// "Session started…"), with the output node's label. Shared by the TUI's
// results screen and the CLI's run summary.

use bnto_core::PipelineDefinition;
use bnto_core::pipeline::{PipelineNode, PipelineResult};

/// Largest output file shown as text; bigger ones aren't a message.
const MAX_MESSAGE_BYTES: u64 = 64 * 1024;

/// The text a message-mode run reports, under its output's label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunMessage {
    /// The output node's label (e.g. "Session Log"), if it has one.
    pub label: Option<String>,
    pub text: String,
}

/// The run's text output as one message, or `None` when nothing it produced
/// reads as text: binary, too big for a message, or only whitespace.
pub fn run_message(result: &PipelineResult, label: Option<String>) -> Option<RunMessage> {
    let parts: Option<Vec<String>> = result.files.iter().map(|f| file_text(&f.data)).collect();
    let text = parts?.join("\n").trim().to_string();
    (!text.is_empty()).then_some(RunMessage { label, text })
}

/// One output file's text, if it's small UTF-8.
fn file_text(data: &bnto_core::processor::FileData) -> Option<String> {
    if data.len().ok()? > MAX_MESSAGE_BYTES {
        return None;
    }
    let bytes = data.to_bytes().ok()?;
    let text = std::str::from_utf8(&bytes).ok()?;
    Some(text.trim_end().to_string())
}

/// The label on the recipe's output node (keep-awake: "Session Log").
pub fn output_label(def: &PipelineDefinition) -> Option<String> {
    find_output(&def.nodes)?
        .params
        .get("label")?
        .as_str()
        .filter(|label| !label.trim().is_empty())
        .map(str::to_string)
}

fn find_output(nodes: &[PipelineNode]) -> Option<&PipelineNode> {
    nodes.iter().find_map(|node| {
        if node.node_type == "output" {
            Some(node)
        } else {
            find_output(node.children.as_deref().unwrap_or_default())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::pipeline::PipelineFileResult;
    use bnto_core::processor::FileData;

    fn output(name: &str, bytes: &[u8]) -> PipelineFileResult {
        PipelineFileResult {
            name: name.into(),
            data: FileData::Bytes(bytes.to_vec()),
            mime_type: "text/plain".into(),
            metadata: serde_json::Map::new(),
        }
    }

    fn result(files: Vec<PipelineFileResult>) -> PipelineResult {
        PipelineResult {
            files,
            duration_ms: 0,
            warnings: Vec::new(),
            node_metadata: Default::default(),
        }
    }

    #[test]
    fn a_text_output_is_the_message() {
        let run = result(vec![output("sh-output", b"No active session.\n")]);
        let message = run_message(&run, Some("Session Log".into())).unwrap();
        assert_eq!(message.text, "No active session.");
        assert_eq!(message.label.as_deref(), Some("Session Log"));
    }

    #[test]
    fn several_outputs_are_joined_in_order() {
        let run = result(vec![output("a", b"first\n"), output("b", b"second")]);
        assert_eq!(run_message(&run, None).unwrap().text, "first\nsecond");
    }

    #[test]
    fn binary_or_empty_output_is_no_message() {
        assert_eq!(
            run_message(&result(vec![output("x", &[0xff, 0xfe])]), None),
            None
        );
        assert_eq!(run_message(&result(vec![output("x", b"  \n")]), None), None);
        assert_eq!(run_message(&result(vec![]), None), None);
    }

    #[test]
    fn an_oversized_output_is_no_message() {
        let big = vec![b'a'; MAX_MESSAGE_BYTES as usize + 1];
        assert_eq!(run_message(&result(vec![output("x", &big)]), None), None);
    }

    #[test]
    fn the_label_comes_from_the_output_node() {
        let json = include_str!("../../bnto-engine/recipes/keep-awake.bnto.json");
        let def: PipelineDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(output_label(&def).as_deref(), Some("Session Log"));
    }
}
