# Metal v0.1.0

## Artifacts
Release artifacts are generated locally and uploaded to GitHub Releases, but are **not stored in this repository**.

Generate artifacts:

```bash
./scripts/build_release_artifacts.sh v0.1.0 linux-x86_64
```

Publish release:

```bash
./scripts/publish_release.sh v0.1.0 linux-x86_64
```

## Highlights
- Loop control fixes (`break`/`next`)
- Mixed int/float comparison improvements
- Safer builtin arity checks
- `check` mode for lex/parse/compile validation
