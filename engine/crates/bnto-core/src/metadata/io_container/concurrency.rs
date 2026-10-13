// Parameters for the `group` and `parallel` containers: how children run.

use super::helpers::{enum_of, min, param, required};
use crate::metadata::{ParameterDef, ParameterType};

/// Parameters for the `group` node.
pub(super) fn group_params() -> Vec<ParameterDef> {
    let modes = [("sequential", "Sequential"), ("parallel", "Parallel")];
    let description =
        "How child nodes execute — sequentially (one after another) or in parallel (concurrently).";
    vec![ParameterDef {
        default: Some(serde_json::json!("sequential")),
        control: Some("select".to_string()),
        ..param("mode", "Mode", description, enum_of(&modes))
    }]
}

/// Parameters for the `parallel` node, in schema order.
pub(super) fn parallel_params() -> Vec<ParameterDef> {
    vec![tasks(), max_workers(), error_strategy()]
}

fn tasks() -> ParameterDef {
    let description = "Array of task definitions to execute concurrently.";
    ParameterDef {
        constraints: Some(required(true)),
        surfaceable: false,
        ..param("tasks", "Tasks", description, ParameterType::Object)
    }
}

fn max_workers() -> ParameterDef {
    let description = "Maximum number of concurrent workers. Defaults to the number of tasks.";
    ParameterDef {
        constraints: Some(min(1.0, false)),
        ..param(
            "maxWorkers",
            "Max Workers",
            description,
            ParameterType::Number,
        )
    }
}

fn error_strategy() -> ParameterDef {
    let strategies = [("failFast", "Fail Fast"), ("collectAll", "Collect All")];
    let description = "How to handle task errors — fail immediately or collect all results.";
    ParameterDef {
        default: Some(serde_json::json!("failFast")),
        control: Some("select".to_string()),
        ..param(
            "errorStrategy",
            "Error Strategy",
            description,
            enum_of(&strategies),
        )
    }
}
