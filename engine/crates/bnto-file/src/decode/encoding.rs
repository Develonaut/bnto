// Text encodings file-decode reads, and decoding a file's text from one.

use base64::Engine as _;
use base64::alphabet;
use base64::engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig};

/// Standard base64 (RFC 4648 §4) with optional padding, so `aGk` and `aGk=`
/// both decode. Trailing bits must be zero, as the RFC requires.
const BASE64: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

/// A text encoding a file's content can be decoded from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Encoding {
    /// Standard alphabet (`+`, `/`). URL-safe (`-`, `_`) would be its own value.
    Base64,
}

impl Encoding {
    /// Every encoding as `(param value, label)`, in the order the editor lists them.
    pub(super) const OPTIONS: [(&'static str, &'static str); 1] = [("base64", "Base64")];

    /// The encoding a param value names, if any.
    pub(super) fn parse(value: &str) -> Option<Self> {
        match value {
            "base64" => Some(Self::Base64),
            _ => None,
        }
    }

    /// The param value naming this encoding.
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::Base64 => "base64",
        }
    }

    /// Decode `text`, ignoring ASCII whitespace (line breaks, indentation)
    /// anywhere in it. On failure, the reason in words.
    pub(super) fn decode(self, text: &[u8]) -> Result<Vec<u8>, String> {
        let compact: Vec<u8> = text
            .iter()
            .copied()
            .filter(|b| !b.is_ascii_whitespace())
            .collect();
        match self {
            Self::Base64 => BASE64.decode(compact).map_err(base64_reason),
        }
    }
}

/// A base64 decode error in words. Offsets are left out: they count from the
/// text with its whitespace removed, so they wouldn't match the file.
fn base64_reason(error: base64::DecodeError) -> String {
    use base64::DecodeError;
    match error {
        DecodeError::InvalidByte(_, b'=') | DecodeError::InvalidPadding => {
            "'=' padding before the end of the text".to_string()
        }
        DecodeError::InvalidByte(_, byte) => invalid_byte(byte),
        DecodeError::InvalidLength(_) => "the text ends partway through a character".to_string(),
        DecodeError::InvalidLastSymbol(_, _) => {
            "its last character has bits set past the end of the data".to_string()
        }
    }
}

/// Why `byte` can't appear in standard base64, with a hint for URL-safe text.
fn invalid_byte(byte: u8) -> String {
    let shown = if byte.is_ascii_graphic() {
        format!("'{}'", byte as char)
    } else {
        format!("byte 0x{byte:02x}")
    };
    let hint = if matches!(byte, b'-' | b'_') {
        " (URL-safe base64 isn't supported; it uses '-' and '_' for '+' and '/')"
    } else {
        ""
    };
    format!("{shown} is not a base64 character{hint}")
}
