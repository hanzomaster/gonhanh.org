# Custom fork maintenance

This repository is a custom fork of Gõ Nhanh that adds shortcut-expansion scope controls:

- master shortcut-expansion toggle;
- separate shortcut-expansion toggles for Vietnamese (V) and English (E) modes;
- official binary updates disabled for custom builds so upstream releases cannot overwrite the customization.

Keep these changes when syncing with the author. The customization starts at commit `b9323f0` on branch `custom/shortcut-scope`.

## Remotes

- `origin`: `hanzomaster/gonhanh.org` (custom fork)
- `upstream`: `khaphanspace/gonhanh.org` (official repository)

## Automated upstream updates

`.github/workflows/upstream-update-pr.yml` checks `upstream/main` daily at 06:17 UTC. When it finds new commits, it merges them into the bot-owned `automation/upstream-sync` branch, runs the shortcut tests, builds the universal custom macOS app, and opens a pull request against `custom/shortcut-scope` only after the build succeeds. If the merge conflicts, it opens a GitHub issue instead.

Review and merge that pull request to accept the update. `.github/workflows/mirror-custom-main.yml` then fast-forwards `main` to the updated custom branch. The workflow artifact is retained for 14 days and can be tested before merging.

The updater can also be checked immediately with:

```bash
gh workflow run upstream-update-pr.yml \
  --repo hanzomaster/gonhanh.org
```

## Manual update fallback

Use this only when the automated merge reports conflicts or GitHub Actions is unavailable:

```bash
git checkout custom/shortcut-scope
git fetch upstream --tags
git rebase upstream/main

cargo test --manifest-path core/Cargo.toml --test shortcut_expansion_settings_test
cargo clippy --manifest-path core/Cargo.toml --all-targets --all-features -- -D warnings

git push --force-with-lease origin custom/shortcut-scope
git push --force-with-lease origin custom/shortcut-scope:main

gh workflow run custom-macos-build.yml \
  --repo hanzomaster/gonhanh.org \
  --ref custom/shortcut-scope
```

Resolve rebase conflicts without dropping the custom shortcut settings, Rust/Swift FFI synchronization, `GoNhanhCustomBuild`, or the custom-build updater guard.

Download the `GoNhanh-custom-macOS` workflow artifact after the build succeeds. Replacing the app preserves preferences because the bundle identifier remains `org.gonhanh.GoNhanh`, but macOS may require Accessibility permission again after an ad-hoc-signed binary changes.

## Custom build

Build locally with full Xcode using:

```bash
scripts/build/macos.sh --custom --skip-accessibility-update
```

The GitHub Actions workflow `.github/workflows/custom-macos-build.yml` is the fallback when full Xcode is unavailable locally. Do not install an official upstream DMG over the custom app.
