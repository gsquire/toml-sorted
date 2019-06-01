# toml-sorted
This is a cargo sub-command that takes a manifest as a single argument and checks if various fields
are sorted. It returns 0 if all fields are sorted and 1 otherwise.

## Install
```sh
cargo install toml-sorted
```

## Usage
```sh
cargo toml-sorted /path/to/Cargo.toml
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
