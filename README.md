# podinfo-rs

 Podinfo-rs is a microservice example service written in pure Rust. It is supposed to document my (opinionated) best practices for writing microservices in Rust aimed at running in Kubernetes.

 Podinfo-rs is inspired by [https://github.com/stefanprodan/podinfo](podinfo).

 ## Running from shell

 To run the service locally from your dev environment you need Rust and `cargo` installed. Then run:

 ````bash
 cargo run
 ````

To test the service run:

````
curl -v http://localhost:6666/_z/healthz
````

The call should return a `200 OK`:

````
HTTP/1.1 200 OK
content-length: 0
date: Sat, 08 Jan 2022 15:52:39 GMT
````

## Configuration

The service can be configured via environment variables, exclusively.

| Variable | Description | Default |
| --- | --- | --- |
| PODINFORS_BIND_IP | IP address to bind the service to. Currently only IPV4 supported.  | `127.0.0.1` |
| PODINFORS_BIND_PORT | The port the service should listen on. | `6666` |

## REST API

| Request | Description |
| --- | --- |
| `GET /_z/healthz` | Returns `200 OK` when the REST API is up. |
| `GET /_z/readyz` | Returns `200 OK` when the service is functioning (ready state set to *ready*) or `503 Service Unavailable` when ready state is set to *not ready*. The ready state is *ready* by default. |
| `POST /_z/readyz/enable` | Sets the ready state to *ready*. |
| `POST /_z/readyz/disable` | Sets the ready state to *not ready* |
| `POST /echo` | Echoes the `POST`ed body and it's `Content-Type` if specified. If no `Content-Type` ist given, it will echo the body with a `Content-Type` of `application/octet-stream`. |

## Build Pipelines

The project uses GitHub Actions for the build and release process:

* The [build.yml](.github/workflows/build.yml) pipeline is run on every push and merge request. It checks formatting, uses `cargo clippy` for linting and builds and tests the project and then tries to build the container. The container is build in `--release` mode, and tests are run again.
* The [release.yml](.github/workflows/release.yml) pipeline is run on version tags (semantic versioning compatible) and performs all steps of the build pipeline, but finally pushes the docker image into the GitHub Container Registry.
* The main branch is protected, thus all changes have to be merged into the `main` branch via pull requests. A pull request requires to pass the build pipeline. We also require merge request to be based on an up-to-date `main` with a linea history.

### Open Issues

* It's unclear whether the container build should run tests again.
* The release pipeline should ideally produce a GitHub release in addition to the docker image.
* Untagged builds on main could also produce snapshot images for easier testing.
* How to handle pre-releases?

## CHANGELOG generation

We use [git-cliff](https://github.com/orhun/git-cliff) to generate the CHANGELOG from commits following the [Conventional Commits](https://www.conventionalcommits.org/) standard. The configuration for `git-cliff` is available in [`cliff.toml`](cliff.toml).

## Releases

Releases are performed following a number of manual steps. We use [`cargo-next`](https://github.com/conventional-commits-rs/cargo-next) for managing the version number.

The steps are:

* Check the current version and whether it is the version you want to release (specifically check for breaking changes). You can use `cargo next --get` to get the current version number.
* If required, update the version accordingly, e.g., with something like `cargo next --patch/--minor/--major`.
* Update the `CHANGELOG` with `git cliff --tag <release version>`.
* Commit everything and the tag the commit with `git tag -a -m 'Release <release version>'`.
* Push the commit, wait if everything builds successfully, and then push the tag to trigger the release.

## Automatic Dependency Updates with Renovate

The project uses [Renovate](https://github.com/renovatebot/renovate) to automatically update dependencies using pull requests. We use a fairly standard setup, please have a lokk at [`renovate.json`](renovate.json). Modifications so far:

* Added `:semanticCommits` for semantic commit messages that can be used to automatically generate a CHANGELOG with [`git-chglog`](https://github.com/git-chglog/git-chglog) (or other tools).
