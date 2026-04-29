## 1. GitHub Infrastructure Setup

- [x] 1.1 Create the `.github/workflows/deploy.yml` file with the automation logic.
- [x] 1.2 Configure the GitHub Action to use `cargo-binstall` for fast `dx` installation.
- [x] 1.3 Implement caching for `cargo` and `dx` to minimize build durations.

## 2. Deployment Configuration

- [x] 2.1 Set up the `gh-pages` branch in the repository.
- [x] 2.2 Configure the workflow to use `peaceiris/actions-gh-pages` for the `target/dx/hybrid-racing-simulator/release/web/public/` directory deployment.

## 3. Verification

- [x] 3.1 Run a manual build locally using `dx bundle --web --release` to verify the `target/dx/hybrid-racing-simulator/release/web/public/` output.
- [x] 3.2 Trigger the GitHub Action by pushing to `main` and verify the successful deployment to GitHub Pages.

## 4. Cleanup
- [x] Remove the DEPLOYMENT.md temporary file
- [ ] Only deploy on main


