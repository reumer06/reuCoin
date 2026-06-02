# Contributing to reuCoin

Thank you for your interest in contributing to reuCoin! This is an educational project and we welcome contributions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/yourusername/reuCoin.git
   cd reuCoin
   ```
3. **Create a branch** for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Build and test:**
   ```bash
   cargo build
   cargo test
   ```

## Development Workflow

### Code Style

- Follow standard Rust conventions (rustfmt)
- Run `cargo fmt` before committing
- Use meaningful variable and function names
- Add comments for complex logic

### Before Submitting

```bash
# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings

# Build in release mode
cargo build --release

# Run tests (if available)
cargo test
```

### Commit Messages

Use clear, descriptive commit messages:
- ✅ Good: `feat: add transaction signing in wallet`
- ✅ Good: `fix: handle connection timeout in node`
- ❌ Bad: `fix stuff`
- ❌ Bad: `update`

## Areas for Contribution

### Easy (Good for Beginners)

- [ ] Add documentation to public APIs
- [ ] Improve error messages
- [ ] Add comments to complex code sections
- [ ] Create more examples in documentation
- [ ] Improve shell completion files

### Medium

- [ ] Add comprehensive error handling
- [ ] Implement transaction validation
- [ ] Add integration tests
- [ ] Improve logging
- [ ] Optimize performance

### Advanced

- [ ] Implement fork resolution
- [ ] Add block pruning
- [ ] Implement SPV (Simplified Payment Verification)
- [ ] Add REST API
- [ ] Implement network security improvements

## Project Structure Understanding

```
├── lib/      - Core blockchain logic (most critical)
├── node/     - P2P network node
├── miner/    - Mining client
└── wallet/   - User-facing wallet
```

### Key Files

- `lib/src/types.rs` - Core data structures
- `lib/src/crypto.rs` - Cryptographic operations
- `node/src/handler.rs` - Transaction/block handling
- `lib/src/network.rs` - Network protocol

## Testing

If you add new functionality, please include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let input = /* ... */;
        
        // Act
        let result = your_function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

Run tests with:
```bash
cargo test
```

## Reporting Issues

### Bug Reports

Include:
- Clear title: "Wallet crashes on invalid amount"
- What you tried
- What happened
- What should happen
- Steps to reproduce
- Your system info (OS, Rust version)

### Feature Requests

Include:
- Clear description of the feature
- Why it would be useful
- Example usage
- Implementation notes (if any)

## Documentation

If you modify public APIs, please update:
- Code comments with `///` doc comments
- README.md if it changes user-facing behavior
- Examples directory if applicable

Example:
```rust
/// Validates a block header against the current target difficulty
/// 
/// # Arguments
/// * `header` - The block header to validate
/// 
/// # Returns
/// * `Ok(())` if header is valid
/// * `Err(error)` if validation fails
pub fn validate_block_header(header: &BlockHeader) -> Result<()> {
    // implementation
}
```

## Performance Considerations

When submitting performance-related changes:
- Include benchmarks showing the improvement
- Test with larger datasets (if applicable)
- Don't sacrifice readability for marginal gains

## Security

- **Don't commit private keys** or sensitive data
- Report security issues privately to maintainers
- Don't merge unreviewed security changes

## Review Process

1. Create a Pull Request with clear description
2. Code review by maintainers
3. CI/CD checks must pass
4. Address feedback
5. Merge! 🎉

## Code Review Guidelines

When reviewing others' code:
- Be respectful and constructive
- Ask questions if unclear
- Suggest improvements, don't demand
- Acknowledge good solutions

## Questions?

- Check existing issues and PRs
- Read the README and QUICKSTART
- Look at existing code patterns
- Ask in the discussion section

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing! Every contribution helps improve reuCoin! 🙏**

