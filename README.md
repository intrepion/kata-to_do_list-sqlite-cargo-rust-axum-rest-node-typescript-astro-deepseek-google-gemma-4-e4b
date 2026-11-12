# kata-to_do_list-sqlite-cargo-rust-axum-rest-node-typescript-astro-deepseek-google-gemma-4-e4b

## Continuous integration

CI runs the existing Rust manifest without suppressing errors. The current baseline is incomplete: Cargo.toml declares no target and server/src/main.rs contains unfinished setup code. This PR exposes those blockers; it does not implement the missing application.

The workflows are read-only and do not deploy. Action dependencies are recorded in
`.github/workflows/actions.lock`. After changing action references, run
`gh actions-lock` and `gh actions-lock --verify` with official
`github/gh-actions-lock` v0.4.1, then commit the workflow and lockfile together.
