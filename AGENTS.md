# Custom fork maintenance

This repository is a custom fork of Gõ Nhanh that adds shortcut-expansion scope controls:

- master shortcut-expansion toggle;
- separate shortcut-expansion toggles for Vietnamese (V) and English (E) modes;
- official binary updates disabled for custom builds so upstream releases cannot overwrite the customization.

Keep these changes when syncing with the author. The customization starts at commit `b9323f0` on branch `custom/shortcut-scope`.

## Remotes

- `origin`: `hanzomaster/gonhanh.org` (custom fork)
- `upstream`: `khaphanspace/gonhanh.org` (official repository)

## Updating from an official release

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
