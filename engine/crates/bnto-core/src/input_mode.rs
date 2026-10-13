//! `InputMode` — how a recipe receives its input, plus its wire-string form
//! (the input node's `mode` param: `file-upload`, `url`, `text`, `none`).

use std::fmt;
use std::str::FromStr;

/// How the recipe expects to receive its input data.
/// Read from the input node's `mode` parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    /// User uploads files (default). CLI reads file paths from disk.
    #[default]
    FileUpload,
    /// User provides a URL. CLI accepts a URL string.
    Url,
    /// User provides text content. CLI accepts a text string.
    Text,
    /// Recipe takes no input at all — all configuration comes from node
    /// fields. Used by utility recipes (session managers, system toggles)
    /// where there is nothing to feed in.
    None,
}

impl InputMode {
    /// The wire string used for the input node's `mode` param.
    pub fn as_str(self) -> &'static str {
        match self {
            InputMode::FileUpload => "file-upload",
            InputMode::Url => "url",
            InputMode::Text => "text",
            InputMode::None => "none",
        }
    }
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for InputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file-upload" => Ok(InputMode::FileUpload),
            "url" => Ok(InputMode::Url),
            "text" => Ok(InputMode::Text),
            "none" => Ok(InputMode::None),
            other => Err(format!("unknown input mode: {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [(InputMode, &str); 4] = [
        (InputMode::FileUpload, "file-upload"),
        (InputMode::Url, "url"),
        (InputMode::Text, "text"),
        (InputMode::None, "none"),
    ];

    #[test]
    fn as_str_returns_wire_string() {
        for (mode, wire) in ALL {
            assert_eq!(mode.as_str(), wire);
        }
    }

    #[test]
    fn display_matches_as_str() {
        for (mode, wire) in ALL {
            assert_eq!(mode.to_string(), wire);
        }
    }

    #[test]
    fn from_str_round_trips() {
        for (mode, wire) in ALL {
            assert_eq!(wire.parse::<InputMode>(), Ok(mode));
        }
    }

    #[test]
    fn from_str_rejects_unknown_mode() {
        let err = "carrier-pigeon".parse::<InputMode>().unwrap_err();
        assert_eq!(err, "unknown input mode: carrier-pigeon");
    }
}
