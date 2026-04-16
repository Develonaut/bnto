// Debug mode — print telemetry events to stderr when BNTO_TELEMETRY_DEBUG=1.

use super::events::Event;

/// Check if telemetry debug mode is enabled.
pub fn is_debug() -> bool {
    std::env::var("BNTO_TELEMETRY_DEBUG").is_ok_and(|v| v == "1")
}

/// Print an event to stderr for debugging.
pub fn debug_event(event: &Event) {
    eprintln!("[telemetry:debug] {} {:?}", event.name, event.properties);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn debug_disabled_by_default() {
        let _guard = ENV_LOCK.lock().unwrap();
        let saved = std::env::var("BNTO_TELEMETRY_DEBUG").ok();
        // SAFETY: tests run serially under ENV_LOCK.
        unsafe { std::env::remove_var("BNTO_TELEMETRY_DEBUG") };

        assert!(!is_debug());

        if let Some(v) = saved {
            unsafe { std::env::set_var("BNTO_TELEMETRY_DEBUG", v) };
        }
    }

    #[test]
    fn debug_enabled_with_1() {
        let _guard = ENV_LOCK.lock().unwrap();
        let saved = std::env::var("BNTO_TELEMETRY_DEBUG").ok();
        unsafe { std::env::set_var("BNTO_TELEMETRY_DEBUG", "1") };

        assert!(is_debug());

        unsafe {
            match saved {
                Some(v) => std::env::set_var("BNTO_TELEMETRY_DEBUG", v),
                None => std::env::remove_var("BNTO_TELEMETRY_DEBUG"),
            }
        }
    }

    #[test]
    fn debug_not_enabled_with_other_values() {
        let _guard = ENV_LOCK.lock().unwrap();
        let saved = std::env::var("BNTO_TELEMETRY_DEBUG").ok();
        unsafe { std::env::set_var("BNTO_TELEMETRY_DEBUG", "true") };

        assert!(!is_debug());

        unsafe {
            match saved {
                Some(v) => std::env::set_var("BNTO_TELEMETRY_DEBUG", v),
                None => std::env::remove_var("BNTO_TELEMETRY_DEBUG"),
            }
        }
    }
}
