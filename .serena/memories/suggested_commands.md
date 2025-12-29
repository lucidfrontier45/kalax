# Essential Commands for Kalax Development

## Build Commands
```bash
cargo build                    # Debug build
cargo build --release         # Release build
cargo check                   # Check for compilation errors without building
```

## Test Commands
```bash
cargo test                    # Run all tests
cargo test <test_name>        # Run specific test (e.g., cargo test it_works)
cargo test --lib              # Run only library tests
cargo test --doc              # Run documentation tests
cargo test -- --nocapture     # Show println! output in tests
```

## Lint & Format Commands
```bash
cargo clippy                  # Run Clippy linter
cargo clippy --fix            # Auto-fix Clippy suggestions
cargo fmt                     # Format code with rustfmt
cargo fmt --check             # Check formatting without modifying files
```

## Documentation Commands
```bash
cargo doc                     # Generate documentation
cargo doc --open              # Generate and open documentation in browser
```

## Git Commands
```bash
git status                    # Check working directory status
git add .                     # Stage all changes
git commit -m "message"       # Commit with message
git push                      # Push to remote
git pull                      # Pull from remote
```

## Utility Commands
```bash
ls -la                        # List files with details
cd <directory>               # Change directory
grep <pattern> <file>        # Search for pattern in file
find . -name "*.rs"          # Find Rust files
cat <file>                   # Display file contents
head <file>                  # Show first lines of file
tail <file>                  # Show last lines of file
```

## Serena Code Analysis Tools
```bash
serena_get_symbols_overview <file>          # Get file structure overview
serena_find_symbol <name>                   # Locate functions/classes
serena_find_referencing_symbols <symbol>    # Find symbol dependencies
serena_search_for_pattern <pattern>         # Flexible pattern search
serena_replace_symbol_body <symbol>         # Precise symbol editing
serena_list_dir <path>                      # Directory exploration
serena_write_memory <name> <content>        # Store project knowledge
serena_read_memory <name>                   # Retrieve stored knowledge
```

## Pre-commit Checklist
Before committing, run:
```bash
cargo fmt --check && cargo clippy && cargo test
```