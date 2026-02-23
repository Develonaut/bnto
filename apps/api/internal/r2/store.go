// Package r2 provides an object storage abstraction for Cloudflare R2.
//
// The ObjectStore interface defines S3-compatible operations needed for
// the file transit layer. Files flow: Browser → R2 → Go API → R2 → Browser.
package r2

import (
	"context"
	"io"
)

// ObjectStore abstracts S3-compatible object storage operations.
// Implementations include the R2 client (production) and in-memory mock (tests).
type ObjectStore interface {
	// ListObjects returns all object keys matching the given prefix.
	ListObjects(ctx context.Context, prefix string) ([]string, error)

	// Download retrieves a single object by key.
	Download(ctx context.Context, key string) (*Object, error)

	// Upload stores content at the given key with the specified content type.
	Upload(ctx context.Context, key string, body io.Reader, contentType string) error
}

// Object represents a retrieved file from the store.
type Object struct {
	Key         string
	Body        io.ReadCloser
	ContentType string
	Size        int64
}

// FileInfo describes a file uploaded to the store by the transit layer.
type FileInfo struct {
	Key         string
	Name        string
	SizeBytes   int64
	ContentType string
}
