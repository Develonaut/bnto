package r2

import (
	"context"
	"fmt"
	"io"
	"mime"
	"os"
	"path/filepath"
	"strings"
)

// DownloadSession downloads all files from uploads/{sessionID}/ to destDir.
// Creates destDir if it doesn't exist.
func DownloadSession(ctx context.Context, store ObjectStore, sessionID, destDir string) error {
	prefix := fmt.Sprintf("uploads/%s/", sessionID)

	keys, err := store.ListObjects(ctx, prefix)
	if err != nil {
		return fmt.Errorf("listing session files %s: %w", sessionID, err)
	}
	if len(keys) == 0 {
		return fmt.Errorf("no files found for session %s", sessionID)
	}

	if err := os.MkdirAll(destDir, 0o755); err != nil {
		return fmt.Errorf("creating destination %s: %w", destDir, err)
	}

	for _, key := range keys {
		if err := ctx.Err(); err != nil {
			return fmt.Errorf("download cancelled: %w", err)
		}
		if err := downloadFile(ctx, store, key, prefix, destDir); err != nil {
			return err
		}
	}

	return nil
}

// UploadOutputs uploads all files from srcDir to executions/{executionID}/output/ in R2.
// Returns metadata for each uploaded file.
func UploadOutputs(ctx context.Context, store ObjectStore, executionID, srcDir string) ([]FileInfo, error) {
	entries, err := os.ReadDir(srcDir)
	if err != nil {
		return nil, fmt.Errorf("reading output dir %s: %w", srcDir, err)
	}

	var files []FileInfo
	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}
		if err := ctx.Err(); err != nil {
			return nil, fmt.Errorf("upload cancelled: %w", err)
		}

		info, err := uploadFile(ctx, store, executionID, srcDir, entry.Name())
		if err != nil {
			return nil, err
		}
		files = append(files, info)
	}

	return files, nil
}

// downloadFile downloads a single object and writes it to destDir.
func downloadFile(ctx context.Context, store ObjectStore, key, prefix, destDir string) error {
	obj, err := store.Download(ctx, key)
	if err != nil {
		return fmt.Errorf("downloading %s: %w", key, err)
	}
	defer obj.Body.Close()

	filename := filepath.Base(strings.TrimPrefix(key, prefix))
	if filename == "" || filename == "." {
		return nil
	}

	destPath := filepath.Join(destDir, filename)
	f, err := os.Create(destPath)
	if err != nil {
		return fmt.Errorf("creating file %s: %w", destPath, err)
	}
	defer f.Close()

	if _, err := io.Copy(f, obj.Body); err != nil {
		return fmt.Errorf("writing file %s: %w", destPath, err)
	}

	return nil
}

// uploadFile uploads a single file from srcDir to R2.
func uploadFile(ctx context.Context, store ObjectStore, executionID, srcDir, filename string) (FileInfo, error) {
	srcPath := filepath.Join(srcDir, filename)

	f, err := os.Open(srcPath)
	if err != nil {
		return FileInfo{}, fmt.Errorf("opening output file %s: %w", srcPath, err)
	}
	defer f.Close()

	stat, err := f.Stat()
	if err != nil {
		return FileInfo{}, fmt.Errorf("stat output file %s: %w", srcPath, err)
	}

	key := fmt.Sprintf("executions/%s/output/%s", executionID, filename)
	contentType := detectContentType(filename)

	if err := store.Upload(ctx, key, f, contentType); err != nil {
		return FileInfo{}, fmt.Errorf("uploading %s: %w", key, err)
	}

	return FileInfo{
		Key:         key,
		Name:        filename,
		SizeBytes:   stat.Size(),
		ContentType: contentType,
	}, nil
}

// detectContentType returns a MIME type based on file extension.
func detectContentType(filename string) string {
	ext := filepath.Ext(filename)
	if ct := mime.TypeByExtension(ext); ct != "" {
		return ct
	}
	return "application/octet-stream"
}
