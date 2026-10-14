// Names a target filesystem can't hold, checked before any move runs.

/// Fail with the reason when `segment` (one file or folder name) can't exist
/// on this OS. Folder inputs are canonical `\\?\` paths on Windows, which skip
/// Win32 name normalization, so these would be created literally or fail
/// halfway through a run.
#[cfg(windows)]
pub fn check_platform_name(segment: &str) -> Result<(), &'static str> {
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if segment.ends_with(['.', ' ']) {
        return Err("ends with a dot or space, which Windows drops");
    }
    if segment
        .chars()
        .any(|c| c.is_control() || "<>:\"|?*".contains(c))
    {
        return Err("holds a character Windows doesn't allow in file names");
    }
    let stem = segment.split('.').next().unwrap_or(segment);
    if RESERVED.iter().any(|r| r.eq_ignore_ascii_case(stem)) {
        return Err("is a name Windows reserves for a device");
    }
    Ok(())
}

/// Fail with the reason when `segment` (one file or folder name) can't exist
/// on this OS. Unix filesystems take any name without `/` or NUL.
#[cfg(not(windows))]
pub fn check_platform_name(segment: &str) -> Result<(), &'static str> {
    if segment.contains('\0') {
        return Err("holds a NUL character");
    }
    Ok(())
}
