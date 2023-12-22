# Maintain

Info regarding crate maintenance.

## Publishing

The README file of the `proc-macro-warning` crate is not found during normal `cargo publish` invocation. We therefore always publish with the `publish` feature, that adapts the path for publishing.

```bash
# Check that it works
cargo publish -p proc-macro-warning --features publish --dry-run
# Actually do the publish
cargo publish -p proc-macro-warning --features publish
```
