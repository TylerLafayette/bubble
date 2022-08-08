# bubble

A container manager built in Rust. Basically a very very very super WIP port of [podman's libpod](https://github.com/containers/podman) to Rust. This is not a direct port due to small stylistic changes and differing goals which are listed below, however most of the core code is directly inspired by the original.

## Differing goals

- Docker compatibility is not a critical priority, and may or may not be
  implemented down the line.
