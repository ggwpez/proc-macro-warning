# Maintain

Info regarding crate maintenance.

## Publishing

The README file of the `proc-macro-warning` crate is not found during normal `cargo publish` invocation. We therefore always publish with the `publish` feature, that adapts the path for publishing.

```bash
# Smoke screen check semver:
cargo semver-checks -p proc-macro-warning
# Replace the version here:
git tag -s -a v1.0.1 -m "Version 1.0.1"
# Check that it works
cargo publish -p proc-macro-warning --dry-run
# Actually do the publish
cargo publish -p proc-macro-warning
```
