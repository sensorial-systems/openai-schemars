# openai-schemars

A Rust utility to generate JSON Schemas compatible with OpenAI's function calling API, using [schemars](https://docs.rs/schemars/) and [serde_json](https://docs.rs/serde_json/).

## Features

- Converts any `schemars::JsonSchema` type to a JSON Schema.
- Enforces OpenAI-compatible subset:
  - Removes unsupported fields (e.g., `format`, `pattern`, etc.).
  - Replaces `oneOf`/`allOf` with `anyOf`.
  - Sets `additionalProperties` to `false` for all objects.
  - Ensures all properties are listed as required.

## Example





