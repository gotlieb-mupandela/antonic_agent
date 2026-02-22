---
name: antonic-agent-orchestrator-npm-publish
description: |
  Publish the antonic-agent-orchestrator npm package with clean git hygiene.

  Triggers when user mentions:
  - "antonic-agent-orchestrator npm publish"
  - "publish antonic-agent-orchestrator"
  - "bump antonic-agent-orchestrator"
---

## Quick usage (already configured)

1. Ensure you are on the default branch and the tree is clean.
2. Bump versions via the shared release bump (this keeps `antonic-agent-orchestrator` aligned with the app/desktop release).

```bash
pnpm bump:patch
# or: pnpm bump:minor
# or: pnpm bump:major
# or: pnpm bump:set -- X.Y.Z
```

3. Commit the bump.
4. Preferred: publish via the "Release App" GitHub Actions workflow by tagging `vX.Y.Z`.

Manual recovery path (sidecars + npm) below.

```bash
pnpm --filter antonic-agent-orchestrator build:sidecars
gh release create antonic-agent-orchestrator-vX.Y.Z packages/orchestrator/dist/sidecars/* \
  --repo Apnium Technology/antonic-agent \
  --title "antonic-agent-orchestrator vX.Y.Z sidecars" \
  --notes "Sidecar binaries and manifest for antonic-agent-orchestrator vX.Y.Z"
```

5. Build antonic-agent-orchestrator binaries for all supported platforms.

```bash
pnpm --filter antonic-agent-orchestrator build:bin:all
```

6. Publish `antonic-agent-orchestrator` as a meta package + platform packages (optionalDependencies).

```bash
node packages/orchestrator/scripts/publish-npm.mjs
```

7. Verify the published version.

```bash
npm view antonic-agent-orchestrator version
```

---

## Scripted publish

```bash
./.opencode/skills/antonic-agent-orchestrator-npm-publish/scripts/publish-antonic-agent-orchestrator.sh
```

---

## First-time setup (if not configured)

Authenticate with npm before publishing.

```bash
npm login
```

Alternatively, export an npm token in your environment (see `.env.example`).

---

## Notes

- `antonic-agent-orchestrator` is published as:
  - `antonic-agent-orchestrator` (wrapper + optionalDependencies)
  - `antonic-agent-orchestrator-darwin-arm64`, `antonic-agent-orchestrator-darwin-x64`, `antonic-agent-orchestrator-linux-arm64`, `antonic-agent-orchestrator-linux-x64`, `antonic-agent-orchestrator-windows-x64` (platform binaries)
- `antonic-agent-orchestrator` is versioned in lockstep with Antonic Agent app/desktop releases.
- antonic-agent-orchestrator downloads sidecars from `antonic-agent-orchestrator-vX.Y.Z` release assets by default.
