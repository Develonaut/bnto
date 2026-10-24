// The `bnto dry-run` command line a declined run suggests: the recipe and the
// run's `-p` values, each quoted so the line pastes into a shell as printed.

/// `bnto dry-run <recipe_arg> -p <param>…`, each argument one shell word.
///
/// Quoting is POSIX sh's, and on Windows PowerShell's (its default shell);
/// cmd.exe has no single quotes, so a quoted word there needs retyping.
pub fn dry_run_command(recipe_arg: &str, params: &[String]) -> String {
    let mut line = format!("bnto dry-run {}", quote(recipe_arg));
    for param in params {
        line.push_str(" -p ");
        line.push_str(&quote(param));
    }
    line
}

/// `arg` bare when every char is plain, else in single quotes.
fn quote(arg: &str) -> String {
    if !arg.is_empty() && arg.chars().all(is_plain) {
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

    fn params(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn a_slug_with_no_params_is_the_plain_command() {
        assert_eq!(
            dry_run_command("download-video", &[]),
            "bnto dry-run download-video"
        );
    }

    #[test]
    fn every_param_follows_its_own_dash_p() {
        let line = dry_run_command(
            "recipes/release-verify.bnto.json",
            &params(&["version=1.2.3", "checks:mode=fast"]),
        );
        assert_eq!(
            line,
            "bnto dry-run recipes/release-verify.bnto.json -p version=1.2.3 -p checks:mode=fast"
        );
    }

    #[test]
    fn a_word_with_a_space_is_single_quoted() {
        let line = dry_run_command("my recipe.bnto.json", &params(&["title=Two words"]));
        assert_eq!(
            line,
            "bnto dry-run 'my recipe.bnto.json' -p 'title=Two words'"
        );
    }

    #[test]
    fn shell_specials_and_empty_values_are_quoted() {
        assert_eq!(quote("a$b"), "'a$b'");
        assert_eq!(quote("a;b"), "'a;b'");
        assert_eq!(quote("a,b"), "'a,b'");
        assert_eq!(quote(""), "''");
    }

    #[cfg(not(windows))]
    #[test]
    fn a_quote_is_escaped_for_sh() {
        assert_eq!(quote("it's"), r"'it'\''s'");
        assert_eq!(quote(r"a\b"), r"'a\b'");
    }

    #[cfg(windows)]
    #[test]
    fn a_quote_is_doubled_for_powershell_and_paths_stay_bare() {
        assert_eq!(quote("it's"), "'it''s'");
        assert_eq!(
            quote(r"C:\recipes\mine.bnto.json"),
            r"C:\recipes\mine.bnto.json"
        );
    }
}
