// Output names for one run, kept apart on every filesystem.
//
// Two result files whose names match, or differ only in case, would land on
// one file on Windows and macOS (an exact match on Linux too), so the later
// one is written as `stem_1.ext`, `stem_2.ext`…, the suffix `file-copy`'s
// `rename` conflict mode uses. Case is folded on every OS so a recipe writes
// the same files everywhere. A result keeps its own name even when the output
// folder holds a file by that name (write mode replaces it), but a suffixed
// name bnto made up never replaces a file there (see `folder`).

mod folder;

use std::collections::HashSet;
use std::path::Path;

use bnto_core::processor::FileData;

/// A result file written under a different name than the pipeline gave it.
#[derive(Debug, Clone, PartialEq)]
pub struct Renamed {
    pub from: String,
    pub to: String,
}

/// The names claimed so far in one run's output folder.
#[derive(Debug, Default)]
pub struct OutputNames {
    taken: HashSet<String>,
}

impl OutputNames {
    /// The name to write `data` under in `dir`: `name` when no earlier claim
    /// has it, else the first suffixed form no claim has and that replaces
    /// nothing in `dir` but a file already holding `data`'s bytes. Either
    /// way it's taken from then on.
    pub fn claim(&mut self, dir: &Path, name: &str, data: &FileData) -> String {
        let mut chosen = name.to_string();
        let mut n = 0;
        while self.taken.contains(&fold(&chosen))
            || (n > 0 && !folder::may_take(dir, &chosen, data))
        {
            n += 1;
            chosen = suffixed(name, n);
        }
        self.taken.insert(fold(&chosen));
        chosen
    }
}

/// Each file's write name in `dir`, in order, and the ones that changed.
pub fn plan<'a>(
    dir: &Path,
    files: impl IntoIterator<Item = (&'a str, &'a FileData)>,
) -> (Vec<String>, Vec<Renamed>) {
    let mut claimed = OutputNames::default();
    let mut chosen = Vec::new();
    let mut renamed = Vec::new();
    for (name, data) in files {
        let to = claimed.claim(dir, name, data);
        if to != name {
            renamed.push(Renamed {
                from: name.to_string(),
                to: to.clone(),
            });
        }
        chosen.push(to);
    }
    (chosen, renamed)
}

/// The key two names share when they'd be one file on some OS.
fn fold(name: &str) -> String {
    name.replace('\\', "/").to_lowercase()
}

/// `name` with `_n` before its extension: `sub/logo.svg` → `sub/logo_1.svg`.
/// A leading dot starts the name, not an extension (`.env` → `.env_1`).
fn suffixed(name: &str, n: usize) -> String {
    let leaf_start = name.rfind(['/', '\\']).map_or(0, |i| i + 1);
    let ext_start = name[leaf_start..]
        .rfind('.')
        .filter(|&i| i > 0)
        .map_or(name.len(), |i| leaf_start + i);
    format!("{}_{n}{}", &name[..ext_start], &name[ext_start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEW: &[u8] = b"BBBB";

    /// The names `names` are written under in an empty folder.
    fn claims(names: &[&str]) -> Vec<String> {
        let dir = tempfile::tempdir().unwrap();
        claims_in(dir.path(), names)
    }

    /// The names `names` are written under in `dir`, each holding `NEW`.
    fn claims_in(dir: &Path, names: &[&str]) -> Vec<String> {
        let data = FileData::Bytes(NEW.to_vec());
        plan(dir, names.iter().map(|n| (*n, &data))).0
    }

    /// A folder holding each `(name, content)` file; names may hold `/`.
    fn folder_with(files: &[(&str, &[u8])]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for (name, content) in files {
            let path = dir.path().join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, content).unwrap();
        }
        dir
    }

    #[test]
    fn test_distinct_names_keep_their_names() {
        assert_eq!(claims(&["a.svg", "b.svg"]), ["a.svg", "b.svg"]);
    }

    #[test]
    fn test_names_differing_only_in_case_are_kept_apart() {
        // Guards: `Logo.svg` and `logo.svg` were one file on Windows, and the
        // second write silently replaced the first.
        assert_eq!(
            claims(&["Logo.svg", "logo.svg"]),
            ["Logo.svg", "logo_1.svg"]
        );
    }

    #[test]
    fn test_an_exact_repeat_is_kept_apart() {
        assert_eq!(
            claims(&["logo.svg", "logo.svg", "LOGO.svg"]),
            ["logo.svg", "logo_1.svg", "LOGO_2.svg"]
        );
    }

    #[test]
    fn test_the_suffix_goes_on_the_file_name_not_its_folder() {
        assert_eq!(
            claims(&["Icons/a.svg", "icons/A.svg"]),
            ["Icons/a.svg", "icons/A_1.svg"]
        );
    }

    #[test]
    fn test_names_in_one_folder_by_either_separator_clash() {
        assert_eq!(
            claims(&["sub/a.svg", "sub\\a.svg"]),
            ["sub/a.svg", "sub\\a_1.svg"]
        );
    }

    #[test]
    fn test_a_name_without_an_extension_takes_the_suffix_at_the_end() {
        assert_eq!(claims(&["README", "readme"]), ["README", "readme_1"]);
        assert_eq!(claims(&[".env", ".ENV"]), [".env", ".ENV_1"]);
    }

    #[test]
    fn test_a_suffixed_name_never_takes_a_later_files_name_from_it() {
        // `a_1.svg` is claimed by the case twin, so the real `a_1.svg` moves on.
        assert_eq!(
            claims(&["a.svg", "A.svg", "a_1.svg"]),
            ["a.svg", "A_1.svg", "a_1_1.svg"]
        );
    }

    #[test]
    fn test_plan_lists_only_the_names_that_changed() {
        let dir = tempfile::tempdir().unwrap();
        let data = FileData::Bytes(NEW.to_vec());
        let files = ["x.svg", "Logo.svg", "logo.svg"].map(|n| (n, &data));
        let (_, renamed) = plan(dir.path(), files);
        assert_eq!(
            renamed,
            [Renamed {
                from: "logo.svg".into(),
                to: "logo_1.svg".into()
            }]
        );
    }

    #[test]
    fn test_one_registry_keeps_names_apart_across_claims() {
        // The progressive writer claims one iteration at a time; the names it
        // picks must match the final write's plan over the whole result.
        let dir = tempfile::tempdir().unwrap();
        let data = FileData::Bytes(NEW.to_vec());
        let mut names = OutputNames::default();
        let progressive = [
            names.claim(dir.path(), "Logo.svg", &data),
            names.claim(dir.path(), "logo.svg", &data),
        ];
        assert_eq!(progressive.to_vec(), claims(&["Logo.svg", "logo.svg"]));
    }

    // --- files already in the output folder ---

    #[test]
    fn test_a_made_up_name_skips_a_file_already_in_the_folder() {
        // RED (data loss): the second `b.txt` was written as `b_1.txt` over
        // the user's own `b_1.txt`, which no output was named.
        let dir = folder_with(&[("b_1.txt", b"CCCC")]);
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt"]),
            ["b.txt", "b_2.txt"]
        );
    }

    #[test]
    fn test_a_file_differing_only_in_case_counts_as_there() {
        // RED: `B_1.TXT` is `b_1.txt` on Windows and macOS, so it's skipped
        // on every OS, even holding the same bytes.
        let dir = folder_with(&[("B_1.TXT", NEW)]);
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt"]),
            ["b.txt", "b_2.txt"]
        );
    }

    #[test]
    fn test_a_gap_in_the_existing_suffixes_is_filled() {
        // RED: with `b_1` and `b_3` there, the free numbers are 2, then 4.
        let dir = folder_with(&[("b_1.txt", b"one"), ("b_3.txt", b"three")]);
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt", "b.txt"]),
            ["b.txt", "b_2.txt", "b_4.txt"]
        );
    }

    #[test]
    fn test_a_made_up_name_skips_a_file_in_a_subfolder() {
        // RED: the folder looked in is the one the name lands in.
        let dir = folder_with(&[("sub/b_1.txt", b"CCCC")]);
        assert_eq!(
            claims_in(dir.path(), &["sub/b.txt", "sub/b.txt"]),
            ["sub/b.txt", "sub/b_2.txt"]
        );
    }

    #[test]
    fn test_a_made_up_name_skips_a_folder_by_that_name() {
        // RED: a write onto a folder fails; the next number is free.
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("b_1.txt")).unwrap();
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt"]),
            ["b.txt", "b_2.txt"]
        );
    }

    #[test]
    fn test_a_made_up_name_skips_a_same_sized_file_with_other_bytes() {
        // RED: same length as `NEW`, so only the bytes tell them apart.
        let dir = folder_with(&[("b_1.txt", b"CCCC")]);
        let src = folder_with(&[("b.txt", NEW)]);
        let data = FileData::Path(src.path().join("b.txt"));
        let files = ["b.txt", "b.txt"].map(|n| (n, &data));
        assert_eq!(plan(dir.path(), files).0, ["b.txt", "b_2.txt"]);
    }

    #[test]
    fn test_a_made_up_name_may_take_a_file_holding_the_same_bytes() {
        // Guards: a rerun, or the final write after a progressive one, finds
        // its own `b_1.txt` there. Replacing it with the same bytes loses
        // nothing, so the run keeps writing `b_1.txt`, not `b_2.txt`.
        let dir = folder_with(&[("b_1.txt", NEW)]);
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt"]),
            ["b.txt", "b_1.txt"]
        );
        let src = folder_with(&[("b.txt", NEW)]);
        let data = FileData::Path(src.path().join("b.txt"));
        let files = ["b.txt", "b.txt"].map(|n| (n, &data));
        assert_eq!(plan(dir.path(), files).0, ["b.txt", "b_1.txt"]);
    }

    #[cfg(unix)]
    #[test]
    fn test_a_made_up_name_skips_a_file_in_a_folder_it_cannot_list() {
        // RED (data loss): with write but no read permission the listing
        // came back empty, so the user's `b_1.txt` was replaced.
        use std::os::unix::fs::PermissionsExt;
        let dir = folder_with(&[("b_1.txt", b"CCCC")]);
        let set = |mode| {
            let mode = std::fs::Permissions::from_mode(mode);
            std::fs::set_permissions(dir.path(), mode).unwrap();
        };
        set(0o300);
        let claimed = claims_in(dir.path(), &["b.txt", "b.txt"]);
        set(0o700);
        assert_eq!(claimed, ["b.txt", "b_2.txt"]);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_a_made_up_name_skips_a_file_spelled_in_another_unicode_form() {
        // RED (data loss): APFS takes the NFD `café_1.txt` there as the NFC
        // name the run makes up, but the listing's spelling didn't match.
        let dir = folder_with(&[("cafe\u{301}_1.txt", b"CCCC")]);
        assert_eq!(
            claims_in(dir.path(), &["caf\u{e9}.txt", "caf\u{e9}.txt"]),
            ["caf\u{e9}.txt", "caf\u{e9}_2.txt"]
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_a_made_up_name_skips_a_link_even_to_the_same_bytes() {
        // RED: only a regular file holding the bytes may be taken; a write
        // through a link would land on whatever it points at.
        let dir = folder_with(&[("target.txt", NEW)]);
        let link = dir.path().join("b_1.txt");
        std::os::unix::fs::symlink(dir.path().join("target.txt"), link).unwrap();
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b.txt"]),
            ["b.txt", "b_2.txt"]
        );
    }

    #[test]
    fn test_a_results_own_name_still_replaces_a_file_by_that_name() {
        // Guards write mode's contract: only a file with the same name as
        // an output is replaced, and that one is.
        let dir = folder_with(&[("b.txt", b"old"), ("b_1.txt", b"old")]);
        assert_eq!(
            claims_in(dir.path(), &["b.txt", "b_1.txt"]),
            ["b.txt", "b_1.txt"]
        );
    }
}
