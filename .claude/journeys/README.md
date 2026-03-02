# User Journey Test Matrices

Living verification contracts for each major domain. Each matrix defines the user journeys that must work, the system gates involved, and what "pass" means.

**These are not test plans.** They're the specification that test plans implement. When an agent writes integration tests for a domain, the matrix is the source of truth for what to test and what success looks like.

## Domains

| Domain | File | What It Covers |
|--------|------|---------------|
| **Auth** | [auth.md](auth.md) | Sign-up, sign-in, sign-out, session lifecycle, access control |
| **Engine** | [engine.md](engine.md) | CLI execution, node types, fixtures, validation, error handling |
| **API** | [api.md](api.md) | Go HTTP API on Railway — endpoints, request/response contracts, R2 file transit |
| **Web App** | [web.md](web.md) | Next.js app — page loads, SEO, tool pages, navigation, responsive behavior |

## How to Use

- **Before writing tests:** Read the matrix for the domain you're testing. Every test case maps to a row.
- **Before shipping a feature:** Check if the feature touches any journey. If it does, verify the affected tests still pass.
- **When adding a journey:** Add it to the matrix first, then write the test. The matrix is the spec.
- **Cross-domain journeys:** Some journeys span domains (e.g., "authenticated user runs a cloud bnto" touches Auth + API + Web). The primary domain owns the journey; other domains are referenced.

## Conventions

- **Journey IDs** are prefixed by domain: `A` (auth), `E` (engine), `P` (API), `W` (web)
- **Pass criteria** describe what success looks like, not how to implement the test
- **Gate maps** document every system checkpoint in the flow — the functions, the auth checks, the error paths
- Matrices are updated when the system changes. Stale matrices are worse than no matrices.
