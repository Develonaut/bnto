// Simple dotenv parser — loads KEY=VALUE pairs from `.env` files.
//
// Intentionally minimal (no crate dependency). Handles comments, blank lines,
// quoted values, and optional `export` prefix. No shell expansion, no
// multiline values. See strategy/recipe-secrets.md for the design rationale.

use std::collections::HashMap;

/// Parse dotenv file contents into key-value pairs.
///
/// Supports: `KEY=VALUE`, `KEY="quoted value"`, `KEY='single quoted'`,
/// `export KEY=VALUE`, `# comments`, blank lines.
pub fn parse_dotenv(contents: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in contents.lines() {
        let trimmed = line.trim();

        // Skip blanks and comments.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Strip optional `export ` prefix for shell compatibility.
        let stripped = trimmed.strip_prefix("export ").unwrap_or(trimmed);

        let Some((key, raw_value)) = stripped.split_once('=') else {
            continue;
        };

        let key = key.trim();
        if key.is_empty() {
            continue;
        }

        let value = unquote(raw_value.trim());
        result.insert(key.to_string(), value);
    }
    result
}

/// Remove surrounding quotes (double or single) from a value string.
fn unquote(s: &str) -> String {
    if s.len() >= 2
        && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
    {
        return s[1..s.len() - 1].to_string();
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_key_value() {
        let env = parse_dotenv("API_KEY=sk-abc123");
        assert_eq!(env.get("API_KEY").unwrap(), "sk-abc123");
    }

    #[test]
    fn multiple_pairs() {
        let env = parse_dotenv("A=1\nB=2\nC=3");
        assert_eq!(env.len(), 3);
        assert_eq!(env["A"], "1");
        assert_eq!(env["B"], "2");
        assert_eq!(env["C"], "3");
    }

    #[test]
    fn comments_and_blanks_skipped() {
        let input = "# This is a comment\n\nKEY=value\n  # Another comment\n  \n";
        let env = parse_dotenv(input);
        assert_eq!(env.len(), 1);
        assert_eq!(env["KEY"], "value");
    }

    #[test]
    fn double_quoted_value() {
        let env = parse_dotenv(r#"SECRET="hello world""#);
        assert_eq!(env["SECRET"], "hello world");
    }

    #[test]
    fn single_quoted_value() {
        let env = parse_dotenv("SECRET='hello world'");
        assert_eq!(env["SECRET"], "hello world");
    }

    #[test]
    fn export_prefix_stripped() {
        let env = parse_dotenv("export API_KEY=sk-abc123");
        assert_eq!(env["API_KEY"], "sk-abc123");
    }

    #[test]
    fn export_with_quotes() {
        let env = parse_dotenv(r#"export SECRET="my value""#);
        assert_eq!(env["SECRET"], "my value");
    }

    #[test]
    fn value_with_equals_sign() {
        let env = parse_dotenv("URL=https://example.com?foo=bar");
        assert_eq!(env["URL"], "https://example.com?foo=bar");
    }

    #[test]
    fn empty_value() {
        let env = parse_dotenv("EMPTY=");
        assert_eq!(env["EMPTY"], "");
    }

    #[test]
    fn whitespace_around_key_and_value() {
        let env = parse_dotenv("  KEY  =  value  ");
        assert_eq!(env["KEY"], "value");
    }

    #[test]
    fn line_without_equals_skipped() {
        let env = parse_dotenv("NOEQUALS\nKEY=value");
        assert_eq!(env.len(), 1);
        assert_eq!(env["KEY"], "value");
    }

    #[test]
    fn empty_key_skipped() {
        let env = parse_dotenv("=value");
        assert!(env.is_empty());
    }

    #[test]
    fn empty_input() {
        let env = parse_dotenv("");
        assert!(env.is_empty());
    }

    #[test]
    fn quoted_empty_value() {
        let env = parse_dotenv(r#"KEY="""#);
        assert_eq!(env["KEY"], "");
    }

    #[test]
    fn mixed_content() {
        let input = "\
# Database config
DB_HOST=localhost
DB_PORT=5432
export DB_PASS=\"s3cret\"

# API keys
API_KEY='sk-test-123'
";
        let env = parse_dotenv(input);
        assert_eq!(env.len(), 4);
        assert_eq!(env["DB_HOST"], "localhost");
        assert_eq!(env["DB_PORT"], "5432");
        assert_eq!(env["DB_PASS"], "s3cret");
        assert_eq!(env["API_KEY"], "sk-test-123");
    }
}
