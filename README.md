# Example Tauri App

Proof of concept [tauri](https://tauri.app) native desktop 
application with a react frontend, a graphql interface and sqlite for
storage.

## Development Setup

### Prerequisites
Tauri has a [guide][prerequisites] for setting up the basic 
dependencies for tauri apps, including the rust toolchain.
Additionally, since this example app uses 
[next.js](https://nextjs.org/), [node.js](https://nodejs.org/en/) must
be installed.

### Development

In order to run the app in development mode (including hot reload),
execute the following command from the root directory of this
repository:

```bash
cargo tauri dev
```

### Tests
Backend tests are run from the root directory of this repository with
the following command:

```bash
cargo test --manifest-path=src/Cargo.toml -- --test-threads=1
```

[prerequisites]: https://tauri.app/v1/guides/getting-started/prerequisites