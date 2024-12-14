import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: "schema.graphql",
  overwrite: true,
  documents: [],
  ignoreNoDocuments: true,
  generates: {
    "gql/": {
      preset: "client",
      plugins: []
    }
  }
};

export default config;