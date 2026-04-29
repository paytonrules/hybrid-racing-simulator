## Context

The project needs a reliable, automated way to deploy the Dioxus web application to GitHub Pages. A GitOps approach is desired to ensure that the live version always reflects the `main` branch.

## Goals / Non-Goals

**Goals:**
- Implement a GitHub Actions workflow that triggers on every push to `main`.
- Use `dx bundle --web --release` to build the production-ready assets.
- Automatically update the `gh-pages` branch with the contents of the `dist/` directory.
- Ensure the build environment has the necessary Rust and Dioxus CLI dependencies.

**Non-Goals:**
- Deploying to any platform other than GitHub Pages.
- Implementing a custom backend or server-side logic (this is a client-only SPA deployment).
- Complex asset management or external CDN integration (this is out of scope for the initial setup).

## Decisions

**Decision: Use GitHub Actions with `peaceiris/actions-gh-pages`**
- **Rationale**: This is the most standard and well-supported way to deploy static assets from a GitHub workflow to the `gh-pages` branch. It simplifies the "deployment" step of the pipeline significantly.
- **Alternatives Considered**: Manual deployment via SSH/FTP (too manual) or using a custom script to push to an S3-compatible bucket (unnecessary complexity for this stage).

**Decision: Leverage `dx bundle --web --release`**
- **Rationale**: Using the Dioxus CLI's built-in bundling command ensures that all the necessary Wasm-bindgen glue and asset processing are handled exactly as they are during local development.

## Risks / Trade-offs

- **[Risk] Large Build Times**: Installing the Rust toolchain and Dioxus CLI on every run can be slow.
  - **Mitigation**: Utilize GitHub Actions caching mechanisms for `cargo` and `dx` installations.
- **[Risk] Build Failure on Dependency Updates**: Changes in the Rust ecosystem or `dx` CLI might break the build.
  - **Mitigation**: Pin the versions of crucial tools where possible or use a robust `cargo-binstall` approach.
