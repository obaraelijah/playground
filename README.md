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

### Export GraphQL SDL to the Frontend 

1. Export the `SDL` from the backend:

   ```bash
   cargo run \
      --bin export_sdl \
      --manifest-path=src/Cargo.toml > frontend/schema.graphql
   ```

   This will create the `frontend/schema.graphql` file with the schema
   definition.

2. Generate types from the `SDL`:

   ```bash
   cd frontend
   mkdir -p gql
   npm run codegen
   ```
   
   This will create native types from the exported `SDL` in the `gql/`
   directory that can be used by the frontend.
  
[prerequisites]: https://tauri.app/v1/guides/getting-started/prerequisites