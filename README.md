# toml-sorted
This is a small command that takes a manifest as a single argument and checks if various fields
are sorted. It returns 0 if all fields are sorted and 1 otherwise. This crate was created in response to
an [RFI here](https://github.com/dtolnay/request-for-implementation/issues/29).

[![Build Status](https://travis-ci.org/gsquire/toml-sorted.svg?branch=master)](https://travis-ci.org/gsquire/toml-sorted)

## Install
```sh
cargo install toml-sorted
```

## Usage
```sh
toml-sorted /path/to/Cargo.toml
```

## Implementation
It currently checks the following fields in a manifest:

- [dependencies]
- [dev-dependencies]
- [build-dependencies]
- [workspace.members]
- [workspace.exclude]

## License
MIT
