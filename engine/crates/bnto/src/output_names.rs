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

use folder::Spot;

/// A result file written under a different name than the pipeline gave it.
#[derive(Debug, Clone, PartialEq)]
pub struct Renamed {
    pub from: String,
    pub to: String,
}

/// Where one result goes: the name it's written under, and whether a file
/// by that made-up name already holds its bytes, so it's left as is.
#[derive(Debug, Clone, PartialEq)]
pub struct Claimed {
    pub name: String,
    pub held: bool,
}

/// The names claimed so far in one run's output folder, and the made-up
/// names a file already there was found to take (not checked again).
#[derive(Debug, Default)]
pub struct OutputNames {
    taken: HashSet<String>,
    occupied: HashSet<String>,
}

impl OutputNames {
    /// Where to put `data` in `dir`: `name` when no earlier claim has it,
    /// else the first suffixed form no claim has and no file in `dir` takes
    /// (one already holding `data`'s bytes is `held`). Taken from then on.
    pub fn claim(&mut self, dir: &Path, name: &str, data: &FileData) -> Claimed {
        let mut n = 0;
        loop {
            let chosen = if n == 0 {
                name.to_string()
            } else {
                suffixed(name, n)
            };
            if let Some(held) = self.free(dir, &chosen, n > 0, data) {
                self.taken.insert(fold(&chosen));
                return Claimed { name: chosen, held };
            }
            n += 1;
        }
    }

    /// `Some(held)` when `chosen` is free for this claim, `None` when it's
    /// taken. Only a `made_up` name is checked against the folder.
    fn free(&mut self, dir: &Path, chosen: &str, made_up: bool, data: &FileData) -> Option<bool> {
        let key = fold(chosen);
        if self.taken.contains(&key) {
            return None;
        }
        if !made_up {
            return Some(false);
        }
        if self.occupied.contains(&key) {
            return None;
        }
        match folder::spot(dir, chosen, data) {
            Spot::Free => Some(false),
            Spot::Held => Some(true),
            Spot::Taken => {
                self.occupied.insert(key);
                None
            }
        }
    }
}

/// Where each file goes in `dir`, in order, and the ones renamed.
pub fn plan<'a>(
    dir: &Path,
    files: impl IntoIterator<Item = (&'a str, &'a FileData)>,
) -> (Vec<Claimed>, Vec<Renamed>) {
    let mut names = OutputNames::default();
    let mut claims = Vec::new();
    let mut renamed = Vec::new();
    for (name, data) in files {
        let claimed = names.claim(dir, name, data);
        if claimed.name != name {
            renamed.push(Renamed {
                from: name.to_string(),
                to: claimed.name.clone(),
            });
        }
        claims.push(claimed);
    }
    (claims, renamed)
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
        names_of(plan(dir, names.iter().map(|n| (*n, &data))).0)
    }

    fn names_of(claims: Vec<Claimed>) -> Vec<String> {
        claims.into_iter().map(|c| c.name).collect()
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
            names.claim(dir.path(), "Logo.svg", &data).name,
            names.claim(dir.path(), "logo.svg", &data).name,
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
        assert_eq!(names_of(plan(dir.path(), files).0), ["b.txt", "b_2.txt"]);
    }

    #[test]
    fn test_a_made_up_name_holding_the_same_bytes_is_taken_and_held() {
        // A rerun, or the final write after a progressive one, finds its own
        // `b_1.txt` there: the run keeps `b_1.txt`, not `b_2.txt`, and marks
        // it held so it's left as is. RED for `held`, a guard for the name.
        let dir = folder_with(&[("b_1.txt", NEW)]);
        let bytes = FileData::Bytes(NEW.to_vec());
        let src = folder_with(&[("b.txt", NEW)]);
        let path = FileData::Path(src.path().join("b.txt"));
        for data in [&bytes, &path] {
            let files = ["b.txt", "b.txt"].map(|n| (n, data));
            let claims = plan(dir.path(), files).0;
            let held: Vec<bool> = claims.iter().map(|c| c.held).collect();
            assert_eq!(held, [false, true]);
            assert_eq!(names_of(claims), ["b.txt", "b_1.txt"]);
        }
    }

    #[test]
    fn test_a_files_own_name_is_never_held() {
        // Guards: a result's own name is written even over the same bytes;
        // only a name bnto made up is checked against the folder.
        let dir = folder_with(&[("b.txt", NEW)]);
        let data = FileData::Bytes(NEW.to_vec());
        let claims = plan(dir.path(), [("b.txt", &data)]).0;
        assert!(!claims[0].held);
    }

    #[test]
    fn test_a_name_a_users_file_took_stays_taken_for_later_claims() {
        // The folder is read once per made-up name a file there takes: a
        // later claim moves past the user's `b_1.txt` without listing or
        // looking it up again, so it stays taken even once the file is gone.
        let dir = folder_with(&[("b_1.txt", b"CCCC")]);
        let data = FileData::Bytes(NEW.to_vec());
        let mut names = OutputNames::default();
        assert_eq!(names.claim(dir.path(), "b.txt", &data).name, "b.txt");
        assert_eq!(names.claim(dir.path(), "b.txt", &data).name, "b_2.txt");
        std::fs::remove_file(dir.path().join("b_1.txt")).unwrap();
        assert_eq!(names.claim(dir.path(), "b.txt", &data).name, "b_3.txt");
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
