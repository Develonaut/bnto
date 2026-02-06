# Cross-Platform Path Resolution

Bento now supports cross-platform path resolution for variables, making it easy to share configuration files across Mac and Windows.

## Overview

When you store paths in `variables.json`, Bento automatically resolves special markers and environment variables, allowing you to write paths that work across different platforms and user accounts.

## Resolution Order

1. **Special Markers** - Platform-agnostic placeholders
2. **Environment Variables** - `${VAR}` or `$VAR` syntax
3. **Platform Overrides** - Optional platform-specific files

## Special Markers

### {{BENTO_HOME}}
Expands to your configured bento home directory.

```json
{
  "MY_BENTOS": "{{BENTO_HOME}}/custom-bentos"
}
```

### {{GDRIVE}}
Auto-detects your Google Drive root path.

**Mac:**
```
/Users/{user}/Library/CloudStorage/GoogleDrive-{email}/My Drive
```

**Windows:**
```
G:\My Drive  (or other drive letter)
```

**Usage:**
```json
{
  "PRODUCTS_URL": "{{GDRIVE}}/Projects/Products"
}
```

### {{DROPBOX}}
Auto-detects your Dropbox root path.

**Mac/Linux:**
```
~/Dropbox
```

**Windows:**
```
%USERPROFILE%\Dropbox
```

**Usage:**
```json
{
  "SHARED_ASSETS": "{{DROPBOX}}/TeamAssets"
}
```

### {{ONEDRIVE}}
Auto-detects your OneDrive root path.

**Mac:**
```
~/Library/CloudStorage/OneDrive-{org}
```

**Windows:**
```
%USERPROFILE%\OneDrive
```

**Usage:**
```json
{
  "WORK_DOCS": "{{ONEDRIVE}}/Documents"
}
```

## Environment Variables

You can reference environment variables using `${VAR}` or `$VAR` syntax:

```json
{
  "PROJECT_ROOT": "${PROJECT_DIR}/source",
  "OUTPUT_DIR": "$HOME/output"
}
```

## Platform-Specific Overrides

For advanced scenarios where markers don't suffice, you can create platform-specific variable files:

- `variables.darwin.json` - Mac-specific overrides
- `variables.windows.json` - Windows-specific overrides
- `variables.linux.json` - Linux-specific overrides

These files are merged with the base `variables.json`, with platform files taking precedence.

### Example Setup

**variables.json** (shared):
```json
{
  "RENDER_THEME": "default",
  "ZOOM_MULTIPLIER": "1.0",
  "PRODUCTS_URL": "{{GDRIVE}}/Products/Miniatures"
}
```

**variables.darwin.json** (Mac-only):
```json
{
  "BLENDER_PATH": "/Applications/Blender.app/Contents/MacOS/Blender"
}
```

**variables.windows.json** (Windows-only):
```json
{
  "BLENDER_PATH": "C:\\Program Files\\Blender Foundation\\Blender\\blender.exe"
}
```

## Real-World Example

The user's actual use case that prompted this feature:

**Before** (platform-specific paths):
```json
{
  "PRODUCTS_URL": "/Users/Ryan/Library/CloudStorage/GoogleDrive-ryanmmchenry@gmail.com/My Drive/Heavy Handed/Products/Miniatures/Bite The Bullet"
}
```

This path only works on Mac. Windows users would need to manually edit it to:
```
G:\My Drive\Heavy Handed\Products\Miniatures\Bite The Bullet
```

**After** (cross-platform):
```json
{
  "PRODUCTS_URL": "{{GDRIVE}}/Heavy Handed/Products/Miniatures/Bite The Bullet"
}
```

This path automatically resolves to the correct platform-specific location on both Mac and Windows.

## Combining Markers and Variables

You can combine multiple resolution types:

```json
{
  "PROJECT_PATH": "{{GDRIVE}}/${PROJECT_NAME}/assets",
  "OUTPUT_PATH": "{{BENTO_HOME}}/output/$USER"
}
```

## Implementation Details

- Resolution happens when variables are **retrieved**, not when they're stored
- This allows editing `variables.json` with markers and seeing the actual paths at runtime
- Failed resolution gracefully falls back to the original value
- Platform detection is automatic - no configuration needed

## Testing

Run tests to verify path resolution:

```bash
go test ./pkg/miso -v -run TestResolvePath
go test ./pkg/miso -v -run TestDetect
```

## Backward Compatibility

This feature is fully backward compatible:
- Absolute paths continue to work as before
- Relative paths continue to work as before
- Only paths with markers or environment variables are transformed
