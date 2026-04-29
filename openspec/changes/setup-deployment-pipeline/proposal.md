## Why

The project currently lacks an automated deployment pipeline, or a live web version. Setting up a GitOps-driven pipeline ensures that every push to `main` is automatically built and deployed to GitHub Pages.

## What Changes

- **Automated Build & Deploy**: Introduction of a GitHub Actions workflow.
- **CI/CD Infrastructure**: Configuration of the `gh-pages` branch for hosting static assets.
- **Build Automation**: Integration of `dx bundle` into the continuous integration process.

## Capabilities

### New Capabilities
- `deployment-pipeline`: Automates the build and deployment of the Dioxus web app to GitHub Pages.

### Modified Capabilities

## Impact

- **GitHub Actions**: New workflow files in `.github/workflows/`.
- **GitHub Repository**: New `gh-pages` branch for hosting.
- **Build Process**: Dependency on `dx` (Dioxus CLI) in the CI environment.
