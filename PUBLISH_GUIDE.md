# Publishing to crates.io Guide

## 📦 Pre-Publication Checklist

✅ Version bumped to 0.3.0  
✅ README.md updated  
✅ Cargo.toml configured  
✅ LICENSE file present  
✅ Library code clean (src/ only)  
✅ All changes committed  
✅ Package test passed  
✅ Documentation in English  

## 🚀 Publishing Steps

### 1. Get API Token

Visit [crates.io/settings/tokens](https://crates.io/settings/tokens) and create a new token.

### 2. Login to crates.io

```bash
cargo login
# Paste your API token when prompted
```

Your token will be saved to `~/.cargo/credentials`

### 3. Final Package Check

```bash
# Verify what will be published
cargo package --list

# Test build the package
cargo package
```

### 4. Publish!

```bash
# Dry run first (recommended)
cargo publish --dry-run

# Actual publish
cargo publish
```

### 5. Verify Publication

Visit your crate page:
- https://crates.io/crates/termux-gui
- https://docs.rs/termux-gui

## 📋 What Gets Published

Only these files are included in the package:

```
termux-gui-0.3.0/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── .gitignore
└── src/
    ├── lib.rs
    ├── error.rs
    ├── connection.rs
    ├── activity.rs
    ├── view.rs
    └── components/
        ├── mod.rs
        ├── button.rs
        ├── text_view.rs
        ├── edit_text.rs
        ├── checkbox.rs
        ├── switch.rs
        ├── radio.rs
        ├── spinner.rs
        ├── layout.rs
        ├── image_view.rs
        ├── progress_bar.rs
        ├── toggle_button.rs
        ├── space.rs
        └── web_view.rs
```

**Total**: ~26 files, library code only  
**Size**: Small and efficient

## ⚠️ Excluded Files

The following are **not** published (per Cargo.toml `exclude`):

- `examples/*` - All example programs
- `*.sh` - Shell scripts
- `AGENTS.md` - Development notes
- `CHANGELOG.md` - Change history
- All WebView documentation
- All comparison docs
- Issue tracking docs
- Build helper scripts

These files remain in the GitHub repository for development purposes.

## 🔄 Publishing Updates

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.3.0): New features, backward compatible
- **PATCH** (0.3.1): Bug fixes, backward compatible

### Update Process

1. Update version in `Cargo.toml`
2. Update `README.md` "What's New" section
3. Commit changes
4. Run `cargo publish`

### Example for 0.3.1 Patch Release

```bash
# 1. Update version
sed -i 's/version = "0.3.0"/version = "0.3.1"/' Cargo.toml

# 2. Update README
# (manually edit What's New section)

# 3. Commit
git add Cargo.toml README.md
git commit -m "chore: bump version to 0.3.1"

# 4. Publish
cargo publish
```

## 📊 Expected Results

After successful publication:

### On crates.io
- Package appears at https://crates.io/crates/termux-gui
- Users can install with `cargo add termux-gui`
- Downloads counter starts tracking

### On docs.rs
- Documentation auto-generated
- Available at https://docs.rs/termux-gui
- API docs browsable online

### Badge Updates
README badges automatically update:
- ![Version](https://img.shields.io/crates/v/termux-gui.svg)
- ![Downloads](https://img.shields.io/crates/d/termux-gui.svg)

## 🐛 Troubleshooting

### "error: failed to verify package"
```bash
# Run package check
cargo package

# Check for errors in build
```

### "error: no upload token found"
```bash
# Login again
cargo login
```

### "error: crate was already uploaded"
```bash
# Version already exists, bump version number
# Edit Cargo.toml version field
```

### "error: failed to build documentation"
```bash
# Test docs locally
cargo doc --no-deps --lib

# Fix any doc warnings
```

## ✅ Post-Publication

### 1. Tag the Release

```bash
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0
```

### 2. GitHub Release

Create a release on GitHub with:
- Tag: v0.3.0
- Title: "v0.3.0 - WebView Complete & Production Ready"
- Description: Copy from "What's New" section

### 3. Announce

Share on:
- r/rust subreddit
- Termux community
- Twitter/X
- Your blog/website

### 4. Monitor

Watch for:
- Download statistics on crates.io
- GitHub issues
- Documentation feedback
- User questions

## 📝 Maintenance

### Regular Updates
- Fix bugs promptly (patch releases)
- Add features (minor releases)
- Keep dependencies updated
- Respond to issues

### Documentation
- Keep README current
- Update examples
- Improve doc comments
- Add tutorials

## 🎯 Success Criteria

✅ Package published successfully  
✅ Documentation builds without errors  
✅ Users can install with `cargo add`  
✅ Examples in README work  
✅ No breaking changes in patch/minor versions  

---

## 🚀 Ready to Publish?

Run these final commands:

```bash
# 1. Login (first time only)
cargo login

# 2. Dry run
cargo publish --dry-run

# 3. Real publish
cargo publish

# 4. Tag release
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0

# 5. Celebrate! 🎉
```

---

**Current Status**: ✅ Ready for publication  
**Version**: 0.3.0  
**Date**: 2024-10-05
