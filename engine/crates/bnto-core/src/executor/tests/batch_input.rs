use super::*;

// =========================================================================
// Batch input — batch nodes receive each file's data as it is (a disk
// reference stays a reference) and the metadata the file carries.
// =========================================================================

/// Emits one file per input, named after it, whose metadata records what
/// the batch node received: the input's metadata plus `was_path`.
struct BatchProbe;

impl crate::processor::NodeProcessor for BatchProbe {
    fn name(&self) -> &str {
        "batch-probe"
    }

    fn metadata(&self) -> crate::metadata::NodeMetadata {
        crate::metadata::NodeMetadata {
            input_cardinality: crate::metadata::InputCardinality::Batch,
            ..crate::processor::NodeProcessor::metadata(&EchoProcessor)
        }
    }

    fn process(
        &self,
        _input: NodeInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        unreachable!("batch-probe only runs as a batch")
    }

    fn process_batch(
        &self,
        input: crate::processor::BatchInput,
        _progress: &ProgressReporter,
        _ctx: &dyn ProcessContext,
    ) -> Result<NodeOutput, BntoError> {
        let files = input
            .files
            .into_iter()
            .map(|f| {
                let mut metadata = f.metadata.clone();
                let was_path = matches!(f.data, FileData::Path(_));
                metadata.insert("was_path".to_string(), was_path.into());
                OutputFile {
                    data: f.data,
                    filename: f.filename,
                    mime_type: "application/octet-stream".to_string(),
                    metadata,
                }
            })
            .collect();
        Ok(NodeOutput {
            files,
            metadata: serde_json::Map::new(),
        })
    }
}

fn run_probe(files: Vec<PipelineFile>) -> Vec<PipelineFile> {
    let def = parse_def(
        r#"{
        "settings": { "iteration": "auto" },
        "nodes": [
            { "id": "in", "type": "input" },
            { "id": "probe", "type": "test-probe" },
            { "id": "out", "type": "output" }
        ]
    }"#,
    );
    let mut registry = mock_registry();
    registry.register("test-probe", Box::new(BatchProbe));
    let reporter = PipelineReporter::new_noop();
    execute_pipeline(&def, files, &registry, &reporter, &NoopContext, fake_now)
        .unwrap()
        .files
        .into_iter()
        .map(|f| PipelineFile {
            name: f.name,
            data: f.data,
            mime_type: f.mime_type,
            metadata: f.metadata,
        })
        .collect()
}

#[test]
fn test_a_batch_node_gets_a_disk_reference_not_bytes() {
    // A path that doesn't exist: reading it would fail the run.
    let file = PipelineFile {
        name: "big.wav".to_string(),
        data: FileData::Path(std::path::PathBuf::from("does-not-exist/big.wav")),
        mime_type: "audio/wav".to_string(),
        metadata: serde_json::Map::new(),
    };
    let out = run_probe(vec![file]);
    assert_eq!(out[0].metadata["was_path"], true);
    assert!(matches!(out[0].data, FileData::Path(_)));
}

#[test]
fn test_a_batch_node_gets_each_files_metadata() {
    let mut a = make_file("a.wav", b"a");
    a.metadata.insert("pack".to_string(), "Guns".into());
    let mut b = make_file("b.wav", b"b");
    b.metadata.insert("pack".to_string(), "Steps".into());
    let out = run_probe(vec![a, b]);
    assert_eq!(out[0].metadata["pack"], "Guns");
    assert_eq!(out[1].metadata["pack"], "Steps");
}
