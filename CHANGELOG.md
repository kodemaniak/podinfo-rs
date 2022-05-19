# Changelog
All notable changes to this project will be documented in this file.

## [0.2.2] - 2022-03-13

### Bug Fixes

- Update rust crate tokio to 1.17.0 (#17)
- Update rust crate anyhow to 1.0.55 (#23)
- Update rust crate tower-http to 0.2.3 (#22)

### Features

- Added build_info crate (#35)
- Provide API for returning runtime information. (#39)

### Miscellaneous Tasks

- Update docker/login-action commit hash to 6af3c11 (#19)
- Update actions/checkout action to v3 (#27)
- Update anchore/scan-action commit hash to 0001ba0 (#29)
- Update docker/login-action commit hash to f6d32ad (#26)
- Update docker/build-push-action commit hash to 309fb91 (#32)

## [0.2.1] - 2022-02-12

### Bug Fixes

- Update rust crate axum to 0.4.5
- Update rust crate tokio to 1.16.1
- Update rust crate serde_json to 1.0.79 (#12)

### Features

- Switch to git-cliff for CHANGELOG generation.

### Miscellaneous Tasks

- Update docker/build-push-action commit hash to 7f9d37f
- Update docker/login-action commit hash to 17f28ab
- Update docker/build-push-action commit hash to fe02965 (#6)

## [0.2.0] - 2022-02-05

### Bug Fixes

- Added full path to `latest` tag.
- Fixed the image name.

## [0.1.0] - 2022-02-05

### Bug Fixes

- Not build twice for every merge request.
- Fixed paths to REST resources. (#10)
- Fixed the ghcr registry path.

### Features

- Added latest and a short version tag (x.y.z, no build metadata).

### Miscellaneous Tasks

- Configured Renovate to use semantic commits.
- Adedd git-chglog configuration for automatic CHANGELOG generation.
- Added Dockerfile and build pipelines plus some docs about the build.
- Prepare 0.1.0 release.

### Refactoring

- Moved z-pages to dedicated module.

<!-- generated by git-cliff -->