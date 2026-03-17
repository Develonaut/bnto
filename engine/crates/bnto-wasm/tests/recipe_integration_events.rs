// Recipe integration tests -- progress event verification.

mod common;

use std::sync::{Arc, Mutex};

use bnto_core::{PipelineEvent, PipelineReporter, execute_pipeline};
use common::{SMALL_JPEG, fake_now, file, parse, real_registry};

#[test]
fn compress_recipe_emits_expected_events() {
    let def = parse(
        r#"{
        "nodes": [
            { "id": "input", "type": "input" },
            {
                "id": "batch-compress", "type": "group",
                "nodes": [{
                    "id": "compress-loop", "type": "loop",
                    "parameters": { "mode": "forEach" },
                    "nodes": [{
                        "id": "compress-image", "type": "image",
                        "parameters": { "operation": "compress", "quality": 80 }
                    }]
                }]
            },
            { "id": "output", "type": "output" }
        ]
    }"#,
    );

    let registry = real_registry();

    // Collect events into a shared vector.
    let events: Arc<Mutex<Vec<PipelineEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let events_clone = Arc::clone(&events);
    let reporter = PipelineReporter::new(move |event: PipelineEvent| {
        events_clone.lock().unwrap().push(event);
    });

    let files = vec![file("photo.jpg", SMALL_JPEG, "image/jpeg")];
    execute_pipeline(&def, files, &registry, &reporter, fake_now)
        .expect("compress pipeline should succeed");

    let collected = events.lock().unwrap();

    assert!(
        matches!(
            collected.first(),
            Some(PipelineEvent::PipelineStarted { .. })
        ),
        "first event should be PipelineStarted"
    );

    assert!(
        matches!(
            collected.last(),
            Some(PipelineEvent::PipelineCompleted { .. })
        ),
        "last event should be PipelineCompleted"
    );

    let group_started = collected.iter().any(
        |e| matches!(e, PipelineEvent::NodeStarted { node_id, .. } if node_id == "batch-compress"),
    );
    assert!(
        group_started,
        "should emit NodeStarted for batch-compress group"
    );
}
