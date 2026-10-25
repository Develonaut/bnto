// The `bnto dry-run` command line a declined run suggests: the recipe and the
// run's `-p` values and `--vars` files, each quoted so the line pastes into a
// shell as printed.

use crate::shell_word::shell_word;

/// What a run or dry run was given that its suggested `bnto dry-run` repeats.
pub struct DryRunHint<'a> {
    /// The recipe slug or path, as typed.
    pub recipe_arg: &'a str,
    params: &'a [String],
    var_files: &'a [String],
}

impl<'a> DryRunHint<'a> {
    /// The hint for `recipe_arg` with the run's `-p` values and `--vars` files.
    pub fn new(recipe_arg: &'a str, params: &'a [String], var_files: &'a [String]) -> Self {
        Self {
            recipe_arg,
            params,
            var_files,
        }
    }

    /// `bnto dry-run <recipe_arg> -p <param>… --vars <file>…`, each argument
    /// one shell word.
    pub fn command(&self) -> String {
        let mut line = format!("bnto dry-run {}", shell_word(self.recipe_arg));
        let flagged = self.params.iter().map(|p| ("-p", p));
        let vars = self.var_files.iter().map(|f| ("--vars", f));
        for (flag, value) in flagged.chain(vars) {
            line.push_str(&format!(" {flag} {}", shell_word(value)));
        }
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    fn dry_run_command(recipe_arg: &str, params: &[String], var_files: &[String]) -> String {
        DryRunHint::new(recipe_arg, params, var_files).command()
    }

    #[test]
    fn a_slug_with_no_params_is_the_plain_command() {
        assert_eq!(
            dry_run_command("download-video", &[], &[]),
            "bnto dry-run download-video"
        );
    }

    #[test]
    fn every_param_follows_its_own_dash_p() {
        let line = dry_run_command(
            "recipes/release-verify.bnto.json",
            &params(&["version=1.2.3", "checks:mode=fast"]),
            &[],
        );
        assert_eq!(
            line,
            "bnto dry-run recipes/release-verify.bnto.json -p version=1.2.3 -p checks:mode=fast"
        );
    }

    #[test]
    fn every_vars_file_follows_its_own_dash_dash_vars() {
        let line = dry_run_command(
            "backup.bnto.json",
            &params(&["mode=fast"]),
            &params(&["machine.json", "my vars.json"]),
        );
        assert_eq!(
            line,
            "bnto dry-run backup.bnto.json -p mode=fast --vars machine.json --vars 'my vars.json'"
        );
    }

    #[test]
    fn a_word_with_a_space_is_single_quoted() {
        let titled = params(&["title=Two words"]);
        let line = dry_run_command("my recipe.bnto.json", &titled, &[]);
        assert_eq!(
            line,
            "bnto dry-run 'my recipe.bnto.json' -p 'title=Two words'"
        );
    }
}
