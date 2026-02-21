# Storage Strategy

**Last Updated:** February 2026
**Status:** Planned -- implementation begins with workflow artifact storage

---

## Guiding Principle

File storage for workflow artifacts, execution outputs, and user-uploaded files for processing. The user's browser does the heavy lifting for client-side processing where possible -- no server-side compute needed for simple transformations.

---

## Storage Options Analysis

### Cloudflare R2

| | Cloudflare R2 | Convex File Storage | AWS S3 |
|---|---|---|---|
| Free storage | 10 GB | 1 GB | 5 GB |
| Egress | **$0 forever** | $0.33/GB | $0.09/GB |
| Storage (paid) | $0.015/GB/mo | $0.03/GB/mo | $0.023/GB/mo |
| CDN included | Yes (Cloudflare) | No | No (CloudFront extra) |

R2's zero egress is the deciding factor for any file-heavy use case. Convex File Storage is useful for tiny transient files if needed, but user-facing content and workflow artifacts should go to R2 or equivalent object storage.

**Decision deferred:** The specific provider will be selected when file storage is needed. R2 is the leading candidate due to zero egress. The architecture below is provider-agnostic.

---

## Use Cases

### Workflow Artifacts & Execution Outputs

Workflows produce output files -- processed images, transformed data, generated reports, API response snapshots. These need to be stored and retrievable.

```
Workflow execution completes
  -> Engine writes output to temp location
  -> API uploads artifacts to object storage
  -> Metadata stored in Convex (storage keys, sizes, types)
  -> UI displays results with download links
```

### User-Uploaded Files for Processing

Users upload files as workflow inputs -- images, CSVs, JSON, documents. These pass through the workflow engine for transformation.

```
User selects file(s) in browser
  -> Client-side validation (type, size)
  -> Upload to object storage (presigned URL or direct)
  -> Workflow references file by storage key
  -> Engine downloads file for processing
  -> Output artifacts stored back to storage
```

### Image Processing (Workflow Node)

For image processing nodes, client-side processing keeps costs at zero:

| Variant | Max Width | Format | Target Size | Use Case |
|---|---|---|---|---|
| `thumb` | 300px | WebP | ~30 KB | Previews, cards |
| `full` | 2400px | WebP | ~400 KB | Full resolution output |

**Client-side processing details:**
- **Library:** `browser-image-compression` (small, well-maintained, handles EXIF rotation)
- **Format:** WebP where supported (97%+ of browsers), JPEG fallback
- **Quality:** 80% for full, 70% for thumbnails
- **Max upload:** Cap original at 20 MB to prevent abuse
- **EXIF:** Strip location data, preserve orientation
- **Web Workers:** Use OffscreenCanvas in a worker for large images to avoid blocking UI

### Why Client-Side Processing Where Possible?

- Client-side = $0 compute cost
- Modern browsers handle this well (Canvas API, OffscreenCanvas)
- No server to scale, no Lambda to pay for
- Tradeoff: slightly inconsistent output across browsers. Acceptable for MVP. If quality variance becomes a problem, add a serverless worker for normalization later.

---

## Cost Projections

### Workflow Artifacts (Primary Use Case)

Assuming average workflow output of ~500 KB per execution:

| Scale | Users | Executions/mo | Storage | Monthly Cost |
|---|---|---|---|---|
| Early (50 users) | 50 | 5,000 | ~2.5 GB | **$0** (free tier) |
| Growing (500 users) | 500 | 50,000 | ~25 GB | **~$0.23/mo** |
| Active (2,000 users) | 2,000 | 200,000 | ~100 GB | **~$1.35/mo** |

Egress: $0 at any scale (with R2).

### Uploaded Files for Processing

| Scale | Users | Files | Storage | Monthly Cost |
|---|---|---|---|---|
| Early (50 users) | 50 | ~500 | ~250 MB | **$0** (free tier) |
| Growing (500 users) | 500 | ~10,000 | ~5 GB | **$0** (free tier) |
| Active (2,000 users) | 2,000 | ~50,000 | ~25 GB | **~$0.23/mo** |

---

## Bucket Organization

```
bnto-storage/
├── artifacts/
│   └── {executionId}/
│       ├── output_{hash}.json
│       ├── output_{hash}.webp
│       └── output_{hash}.csv
├── uploads/
│   └── {userId}/
│       └── {workflowId}/
│           ├── input_{hash}.csv
│           └── input_{hash}.json
└── temp/                          # TTL: 24 hours
    └── {executionId}/
        └── intermediate_{hash}.tmp
```

Single bucket, path-based organization. Keeps it simple. Can split into separate buckets later if access patterns diverge significantly.

---

## Configuration

- **Public access:** Disabled by default. All files accessed via signed URLs.
- **Signed URLs:** Short-lived (1 hour) for downloads. Presigned upload URLs for direct browser uploads.
- **CORS:** Allow uploads from the bnto web app origin.
- **Lifecycle rules:** Temp files auto-delete after 24 hours. Consider Infrequent Access tier for artifacts older than 90 days with low access counts.
- **Size limits:** 20 MB per file for free tier. Higher limits for paid plans.

---

## Cost Summary

| Use Case | Free Tier Runway | Paid Cost at Scale |
|---|---|---|
| Workflow artifacts | ~10,000 executions | ~$1.50/mo at 200K executions |
| Uploaded files | ~10,000 files | ~$0.25/mo at 50K files |
| Combined at maturity | -- | ~$5-10/mo for moderate scale |

With R2's $0 egress, storage costs remain negligible at scale. Well within infrastructure budget.

---

## Decision Log

- **R2 as leading candidate:** 10x free tier over Convex File Storage, $0 egress. Best option for file-heavy workflows.
- **Provider decision deferred:** Architecture is provider-agnostic. Will select when file storage is implemented.
- **Client-side processing:** $0 compute. Browser does the work. Acceptable quality tradeoff for MVP.
- **Single bucket:** Path-based separation. Split later if needed.
- **Signed URLs for all access:** No public files by default. Workflows may process sensitive data.

---

*Review this document when starting file storage implementation. Update cost projections as real usage data comes in.*
