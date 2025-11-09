# Contributing to YEETIFF

Thank you for considering contributing to YEET! This is an **educational and experimental project** designed to help people learn about image formats.

## 🎯 Project Goals

1. **Educational** - Help others learn how image formats work
2. **Experimental** - Test new ideas in compression and encoding
3. **Community-driven** - Welcome contributions from developers of all skill levels
4. **Well-documented** - Every feature should be clearly explained

## 🤝 How to Contribute

### Ways to Contribute

We welcome contributions in many forms:

- 🐛 **Bug reports** - Found a problem? Let us know!
- 💡 **Feature requests** - Have an idea? Share it!
- 📝 **Documentation** - Improve guides, comments, or examples
- 🎨 **Design** - Logo, icons, UI improvements
- 💻 **Code** - Bug fixes, new features, optimizations
- 🧪 **Testing** - Write tests, find edge cases
- 📊 **Benchmarks** - Performance testing and optimization

### Areas We Need Help

#### High Priority

- 🎨 **Logo and icon design** - We need a YEET logo!
- 📸 **ICC color profile support** (v3) - Extract and embed ICC profiles
- 🎬 **Animation support** (v3) - Multi-frame encoding and playback
- 🗜️ **Brotli/Zstd compression** (v3) - Better compression algorithms
- 🧪 **Test coverage** - Unit tests, integration tests

#### Medium Priority

- 📖 **Documentation improvements** - Clearer explanations, more examples
- 🌍 **Cross-platform testing** - macOS and Linux compatibility
- 🚀 **Performance optimization** - Faster encoding/decoding
- 🔧 **CLI improvements** - Better user experience

#### Low Priority

- 🎮 **Animation editor** - GUI tool for creating animations
- 📦 **Package managers** - Distribution via Homebrew, Chocolatey, etc.
- 🌐 **Web viewer** - WASM-based browser viewer

## 🚀 Getting Started

### Prerequisites

**For Rust development:**
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- Code editor (VS Code recommended)

**For Python installer:**
- Python 3.7+
- pip

### Setup Development Environment

1. **Fork and clone:**

```bash
git clone https://github.com/YOUR_USERNAME/YEETIFF
cd yeet-project
```

2. **Build the project:**

```bash
# Build all components
cargo build

# Build specific component
cargo build -p yeet-core

# Build in release mode
cargo build --release
```

3. **Run tests:**

```bash
cargo test --workspace
```

4. **Format and lint:**

```bash
cargo fmt --all
cargo clippy --workspace
```

### Project Structure

```
yeet-project/
├── yeet-core/       # Stable v2 viewer (PRODUCTION)
├── yeet-v3/         # Experimental v3 (DEVELOPMENT)
├── yeet-legacy/     # v1 support (LEGACY)
├── yeet-installer/  # Windows installer (DISTRIBUTION)
├── docs/            # Documentation
└── examples/        # Example files
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed code organization.

## 📋 Contribution Workflow

### 1. Find or Create an Issue

- **Check existing issues** first
- **Create new issue** if needed
- **Discuss approach** before coding (for large features)

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

**Branch naming:**
- `feature/add-icc-support` - New features
- `bugfix/fix-compression-crash` - Bug fixes
- `docs/improve-readme` - Documentation
- `refactor/cleanup-metadata` - Code refactoring

### 3. Make Your Changes

#### Code Style

**Rust:**
- Follow Rust standard style (`cargo fmt`)
- Use `cargo clippy` for linting
- Add doc comments (`///`) for public items
- Keep functions focused and testable

**Python:**
- PEP 8 style guide
- Type hints where applicable
- Docstrings for functions

**Commit messages:**
```
Add ICC profile extraction for PNG files

- Integrate lcms2 library
- Extract embedded ICC profiles
- Add tests for profile parsing

Closes #42
```

### 4. Test Your Changes

**Rust tests:**
```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_compression

# Run with output
cargo test -- --nocapture
```

**Manual testing:**
```bash
# Test conversion
cargo run --bin yeet compile test.png --compress --binary

# Test viewing
cargo run --bin yeet test.yeet

# Test batch
cargo run --bin yeet batch ./test-images
```

### 5. Update Documentation

- Update README if user-facing changes
- Update relevant docs/ files
- Add doc comments to new functions
- Update SPEC files if format changes

### 6. Submit Pull Request

1. **Push to your fork:**
```bash
git push origin feature/your-feature-name
```

2. **Create PR on GitHub**

3. **Fill out PR template:**
   - Describe changes
   - Link related issues
   - Explain testing done
   - Add screenshots if UI changes

4. **Wait for review**

5. **Address feedback**

6. **Merge** (maintainer will merge when approved)

## ✅ Pull Request Checklist

Before submitting:

- [ ] Code compiles without warnings
- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] Branch is up to date with main

## 🧪 Testing Guidelines

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_roundtrip() {
        let original = vec![1, 2, 3, 4, 5];
        let compressed = compress_data(&original);
        let decompressed = decompress_data(&compressed);
        
        assert_eq!(original, decompressed);
    }
}
```

### Test Coverage

We aim for:
- **Core functions:** 80%+ coverage
- **Format parsers:** 90%+ coverage
- **Critical paths:** 100% coverage

### Test Data

Add test images to `examples/`:
- Small files (< 1 MB)
- Various formats (RGB, RGBA)
- Edge cases (1×1, very large)

## 📝 Code Review Process

### What We Look For

- ✅ **Correctness** - Does it work?
- ✅ **Clarity** - Is it easy to understand?
- ✅ **Tests** - Is it tested?
- ✅ **Documentation** - Is it documented?
- ✅ **Style** - Does it follow conventions?
- ✅ **Performance** - Is it efficient?

### Review Timeline

- **Simple PRs:** 1-2 days
- **Complex PRs:** 3-7 days
- **Large features:** 1-2 weeks

We'll provide feedback and work with you to get your changes merged!

## 🐛 Bug Reports

### Good Bug Report

```markdown
**Description:**
YEET crashes when converting large PNG files

**Steps to Reproduce:**
1. Download test image (5000×5000 pixels)
2. Run: `yeet compile large.png --compress`
3. Observe crash

**Expected:**
File converts successfully

**Actual:**
Error: "out of memory"

**Environment:**
- OS: Windows 11
- yeet version: 2.0.0
- RAM: 8 GB

**Additional Context:**
Smaller images work fine. Issue appears around 4000×4000 pixels.
```

### Include

- Clear description
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Error messages/logs
- Sample files (if small)

## 💡 Feature Requests

### Good Feature Request

```markdown
**Feature:** Add support for Brotli compression

**Problem:**
Current zlib compression achieves 50% reduction.
Brotli could improve this to 60-65% for photos.

**Proposed Solution:**
- Integrate `brotli` crate
- Add `--brotli` CLI flag
- Update v3 format flags
- Benchmark against zlib

**Alternatives:**
Could also consider Zstd (faster but less compression)

**References:**
- Brotli: https://github.com/google/brotli
- Benchmark: https://squash.sh
```

### Include

- Clear description
- Problem it solves
- Proposed implementation
- Alternative approaches
- Why it's valuable

## 🎓 Learning Resources

### Image Formats

- [PNG Specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html)
- [How JPEG Works](https://www.youtube.com/watch?v=Kv1Hiv3ox8I)
- [Image Compression Guide](https://developers.google.com/web/fundamentals/performance/optimizing-content-efficiency/image-optimization)

### Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Color Science

- [ICC Profiles](https://www.color.org/icc_specs2.xalter)
- [Color Management](https://www.cambridgeincolour.com/tutorials/color-management1.htm)
- [sRGB Standard](https://en.wikipedia.org/wiki/SRGB)

## 📜 Code of Conduct

### Our Standards

We are committed to providing a welcoming and inclusive experience.

**Expected behavior:**
- ✅ Be respectful and constructive
- ✅ Welcome newcomers and help them learn
- ✅ Accept constructive criticism gracefully
- ✅ Focus on what's best for the community

**Unacceptable behavior:**
- ❌ Harassment or discrimination
- ❌ Trolling or insulting comments
- ❌ Publishing private information
- ❌ Other unprofessional conduct

### Enforcement

Violations will result in:
1. Warning
2. Temporary ban
3. Permanent ban (severe/repeated violations)

Report issues to: [your-email@example.com]

## 🎁 Recognition

### Contributors

All contributors are listed in:
- GitHub Contributors page
- README.md acknowledgments
- Release notes

### Significant Contributions

Major contributions may earn:
- Mention in release announcements
- Co-authorship on format specification
- Collaborator status on repository

## 📞 Getting Help

### Questions?

- **GitHub Discussions:** Ask questions, share ideas
- **Issues:** Bug reports, feature requests
- **Email:** [your-email@example.com]

### Stuck?

Don't hesitate to:
- Ask questions in your PR
- Request guidance in issues
- Reach out to maintainers

We're here to help you succeed! 🎉

## 🗺️ Roadmap

### Current Focus (v2.x)

- Stability improvements
- Bug fixes
- Documentation
- Cross-platform testing

### Future (v3.0)

- ICC color profiles
- Multi-frame animation
- Enhanced compression
- HDR support

### Long-term Vision

- YEET as educational tool in CS courses
- Reference implementation for custom formats
- Community-driven feature development

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## 🙏 Thank You!

Every contribution, no matter how small, helps make YEET better for everyone. We appreciate your time and effort!

**Happy coding!** 🚀

---

**Project Maintainer:** Stijn Jakobs ([@jakobsstijn](https://github.com/jakobsstijn))  
**Repository:** [github.com/jakobsstijn/YEETIFF](https://github.com/jakobsstijn/YEETIFF)  
**License:** MIT
