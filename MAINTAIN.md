# Maintain

Info regarding crate maintenance.

## Publishing

The README file of the `proc-macro-warning` crate is not found during normal `cargo publish` invocation.  
To fix this, we use the `--allow-dirty` flag:

```bash
cp README.md proc-macro-warning/
# Check that it works
cargo publish -p proc-macro-warning --dry-run --allow-dirty
# Actually do the publish
cargo publish -p proc-macro-warning --allow-dirty
```
