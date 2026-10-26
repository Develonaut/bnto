// The one shell-quoting rule for every command line bnto prints to be pasted:
// a declined run's `bnto dry-run` hint and the commands a dry run shows.

/// `arg` as one shell word: bare when every char is plain, else single-quoted.
///
/// Quoting is POSIX sh's, and on Windows PowerShell's (its default shell);
/// cmd.exe has no single quotes, so a quoted word there needs retyping.
/// A leading `=` is quoted too: zsh expands `=word` to that command's path.
pub fn shell_word(arg: &str) -> String {
    if !arg.is_empty() && !arg.starts_with('=') && arg.chars().all(is_plain) {
        return arg.to_string();
    }
    format!("'{}'", arg.replace('\'', QUOTE_ESCAPE))
}

/// Means the same to sh and PowerShell when unquoted (Windows paths keep `\`).
fn is_plain(c: char) -> bool {
    c.is_ascii_alphanumeric() || "-_./:=+".contains(c) || (cfg!(windows) && c == '\\')
}

/// A `'` inside single quotes: sh closes, escapes and reopens; PowerShell doubles it.
#[cfg(not(windows))]
const QUOTE_ESCAPE: &str = r"'\''";
#[cfg(windows)]
const QUOTE_ESCAPE: &str = "''";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_word_stays_bare() {
        assert_eq!(shell_word("version=1.2.3"), "version=1.2.3");
    }

    #[test]
    fn a_leading_equals_is_quoted_for_zsh() {
        // zsh expands a word that starts with `=` to that command's path.
        assert_eq!(shell_word("=1.2"), "'=1.2'");
        assert_eq!(shell_word("version=1.2"), "version=1.2");
    }

    #[test]
    fn shell_specials_and_empty_values_are_quoted() {
        assert_eq!(shell_word("a b"), "'a b'");
        assert_eq!(shell_word("a$b"), "'a$b'");
        assert_eq!(shell_word("a;b"), "'a;b'");
        assert_eq!(shell_word("a*b"), "'a*b'");
        assert_eq!(shell_word("a,b"), "'a,b'");
        assert_eq!(shell_word("{{input}}"), "'{{input}}'");
        assert_eq!(shell_word(""), "''");
    }

    #[cfg(not(windows))]
    #[test]
    fn a_quote_is_escaped_for_sh() {
        assert_eq!(shell_word("it's"), r"'it'\''s'");
        assert_eq!(shell_word(r"a\b"), r"'a\b'");
    }

    #[cfg(windows)]
    #[test]
    fn a_quote_is_doubled_for_powershell_and_paths_stay_bare() {
        assert_eq!(shell_word("it's"), "'it''s'");
        assert_eq!(
            shell_word(r"C:\recipes\mine.bnto.json"),
            r"C:\recipes\mine.bnto.json"
        );
    }
}
