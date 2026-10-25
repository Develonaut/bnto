// The `bnto dry-run` command line a declined run suggests: the recipe and the
// run's `-p` values, each quoted so the line pastes into a shell as printed.

use crate::shell_word::shell_word;

/// `bnto dry-run <recipe_arg> -p <param>…`, each argument one shell word.
pub fn dry_run_command(recipe_arg: &str, params: &[String]) -> String {
    let mut line = format!("bnto dry-run {}", shell_word(recipe_arg));
    for param in params {
        line.push_str(" -p ");
        line.push_str(&shell_word(param));
    }
    line
}

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
}
