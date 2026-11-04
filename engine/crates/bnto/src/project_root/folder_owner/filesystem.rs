//! Whether a folder's filesystem records real ownership. Some mounts make
//! it up: FAT, exFAT, NTFS, HFS, optical media, VM shared folders (9p, as
//! WSL2's `/mnt/c`, and VirtualBox's) and SMB shares take an owner from
//! their mount options, and macOS mounts disk images and external drives
//! `noowners` (`MNT_IGNORE_OWNERSHIP`), so every file shows as the mounting
//! user's and the owner check would pass for files anyone put there. NFS
//! trusts whatever uid a client machine's root sends, and FUSE can be
//! anything, so both count as not recording it.
//! Any error counts as not recording it. Unixes other than macOS and Linux
//! aren't checked here.

use std::path::Path;

/// Whether `path`'s filesystem records who owns each file.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(super) fn records_ownership(path: &Path) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let Ok(c_path) = std::ffi::CString::new(path.as_os_str().as_bytes()) else {
        return false;
    };
    // SAFETY: an all-zero statfs (numbers and byte arrays) is valid to fill.
    let mut fs: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: `c_path` is NUL-terminated and `fs` a valid out pointer.
    if unsafe { libc::statfs(c_path.as_ptr(), &mut fs) } != 0 {
        return false;
    }
    from_statfs(&fs)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub(super) fn records_ownership(_path: &Path) -> bool {
    true
}

/// macOS filesystem types whose owners come from the mount, not the disk,
/// plus NFS (FUSE-T reports as `nfs` too), macFUSE under its names, and
/// `lifs`, which user-space FAT and exFAT report on some releases.
#[cfg(target_os = "macos")]
const APPLE_NO_OWNERS: [&str; 14] = [
    "msdos",
    "exfat",
    "ntfs",
    "smbfs",
    "afpfs",
    "webdav",
    "cd9660",
    "udf",
    "nfs",
    "macfuse",
    "osxfuse",
    "osxfusefs",
    "fusefs",
    "lifs",
];

#[cfg(target_os = "macos")]
fn from_statfs(fs: &libc::statfs) -> bool {
    let name: Vec<u8> = fs
        .f_fstypename
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect();
    apple_records_ownership(fs.f_flags, &String::from_utf8_lossy(&name))
}

/// The macOS rule, on a mount's flags and filesystem type name.
#[cfg(target_os = "macos")]
fn apple_records_ownership(flags: u32, fstype: &str) -> bool {
    flags & libc::MNT_IGNORE_OWNERSHIP as u32 == 0 && !APPLE_NO_OWNERS.contains(&fstype)
}

/// Linux superblock magic numbers (`linux/magic.h`; ntfs3 sets "ntfs",
/// vboxsf its own) whose owners come from mount options, plus NFS and FUSE.
#[cfg(target_os = "linux")]
const LINUX_NO_OWNERS: [u32; 14] = [
    0x4d44,      // MSDOS_SUPER_MAGIC (FAT)
    0x2011_bab0, // EXFAT_SUPER_MAGIC
    0x5346_544e, // NTFS_SB_MAGIC
    0x7366_746e, // ntfs3
    0x517b,      // SMB_SUPER_MAGIC
    0xff53_4d42, // CIFS_SUPER_MAGIC
    0xfe53_4d42, // SMB2_SUPER_MAGIC
    0x6573_5546, // FUSE_SUPER_MAGIC
    0x9660,      // ISOFS_SUPER_MAGIC
    0x1501_3346, // UDF_SUPER_MAGIC
    0x4244,      // HFS_SUPER_MAGIC
    0x0102_1997, // V9FS_MAGIC (9p: WSL2's drvfs, VM shares)
    0x786f_4256, // VBOXSF_SUPER_MAGIC
    0x6969,      // NFS_SUPER_MAGIC
];

#[cfg(target_os = "linux")]
fn from_statfs(fs: &libc::statfs) -> bool {
    // Magic numbers are 32-bit; `f_type`'s width and sign vary by target.
    linux_records_ownership(fs.f_type as u32)
}

/// The Linux rule, on a filesystem's superblock magic number.
#[cfg(target_os = "linux")]
fn linux_records_ownership(magic: u32) -> bool {
    !LINUX_NO_OWNERS.contains(&magic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[test]
    fn a_noowners_mount_or_foreign_filesystem_records_no_owners() {
        let ignore = libc::MNT_IGNORE_OWNERSHIP as u32;
        assert!(apple_records_ownership(0, "apfs"));
        assert!(apple_records_ownership(0x1000, "hfs"));
        assert!(!apple_records_ownership(ignore, "apfs"));
        for fstype in APPLE_NO_OWNERS {
            assert!(!apple_records_ownership(0, fstype), "{fstype}");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn foreign_and_fuse_filesystems_record_no_owners() {
        for magic in [0xef53, 0x9123_683e, 0x5846_5342, 0x0102_1994] {
            assert!(linux_records_ownership(magic), "{magic:#x}"); // ext4, btrfs, xfs, tmpfs
        }
        for magic in LINUX_NO_OWNERS {
            assert!(!linux_records_ownership(magic), "{magic:#x}");
        }
        assert!(!linux_records_ownership(
            0xff53_4d42_u32 as i32 as i64 as u32
        ));
    }

    #[test]
    fn the_test_sandbox_records_owners() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(records_ownership(tmp.path()));
    }

    #[test]
    fn a_missing_folder_records_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("missing");
        assert_eq!(
            records_ownership(&missing),
            cfg!(not(any(target_os = "macos", target_os = "linux")))
        );
    }
}
