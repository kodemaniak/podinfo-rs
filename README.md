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
curl -v http://localhost:6666/healthz
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
| PODINFORS_BIOND_PORT | The port the service should listen on. | `6666` |

## REST API

| Request | Description |
| --- | --- |
| `GET /healthz` | Returns `200 OK` when the REST API is up. |
| `GET /readyz` | Returns `200 OK` when the service is functioning. Currently this is always true, when `/healtz` returns `OK`. |
