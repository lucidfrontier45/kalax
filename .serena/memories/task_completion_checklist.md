# Task Completion Checklist

Before marking a task as complete or committing changes, ensure:

## Code Quality Checks
- [ ] Run `cargo fmt` to format code
- [ ] Run `cargo clippy` to check for linting issues
- [ ] Run `cargo test` to ensure all tests pass
- [ ] Run `cargo doc` to generate documentation

## Serena Tool Usage
- [ ] Use Serena tools for code analysis (prefer `serena_get_symbols_overview`, `serena_find_symbol` over basic file operations)
- [ ] Update Serena memories when project conventions change
- [ ] Use `serena_write_memory` to store important project knowledge

## Import Style Verification
- [ ] Verify no glob imports (`use crate::*`) except for preludes
- [ ] Check import grouping follows VSCode settings (plain prefix, grouped by crate)

## Code Style Compliance
- [ ] Function names use `snake_case` starting with verbs
- [ ] Type names use `PascalCase`
- [ ] Constants use `SCREAMING_SNAKE_CASE`
- [ ] Variables use descriptive `snake_case` names
- [ ] Error handling uses `Result<T, E>` pattern

## Testing Requirements
- [ ] Unit tests written for new public functions
- [ ] Edge cases tested (empty inputs, boundary values)
- [ ] Descriptive test names (e.g., `test_calculate_mean_with_empty_slice`)
- [ ] Error conditions tested appropriately

## Documentation
- [ ] Public APIs documented with `///` comments
- [ ] Parameters, return values, and panics documented
- [ ] Complex functions include examples

## Performance & Safety
- [ ] No unnecessary allocations in hot paths
- [ ] Floating-point edge cases handled (NaN, infinity)
- [ ] Input validation for data ranges
- [ ] No unsafe code without thorough documentation

## Commit Message Guidelines
- Use descriptive commit messages
- Follow conventional commit format when possible
- Keep commits focused and atomic
- Update documentation with code changes

## Final Verification
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Documentation generates successfully
- [ ] No Clippy warnings remain