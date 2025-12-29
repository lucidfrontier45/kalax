# Serena Code Analysis Tools Guide

## Overview
Serena provides intelligent code analysis and exploration tools for efficient development. Prioritize Serena tools over basic file operations for better efficiency.

## Symbol Analysis Tools
- `serena_get_symbols_overview <file>` - Get high-level overview of symbols in a file
- `serena_find_symbol <name_path_pattern>` - Locate functions, classes, or other symbols
- `serena_find_referencing_symbols <name_path> <file>` - Find all references to a specific symbol

## Code Search Tools
- `serena_search_for_pattern <substring_pattern>` - Flexible pattern matching across files
- `serena_find_file <file_mask> <relative_path>` - Find files by pattern

## Symbol Editing Tools
- `serena_replace_symbol_body <name_path> <relative_path> <body>` - Replace entire symbol body
- `serena_insert_after_symbol <name_path> <relative_path> <body>` - Insert code after symbol
- `serena_insert_before_symbol <name_path> <relative_path> <body>` - Insert code before symbol
- `serena_rename_symbol <name_path> <relative_path> <new_name>` - Rename symbols across codebase

## Project Navigation Tools
- `serena_list_dir <relative_path>` - List files and directories
- `serena_activate_project <project>` - Activate a project for analysis

## Memory System Tools
- `serena_write_memory <memory_file_name> <content>` - Store project knowledge
- `serena_read_memory <memory_file_name>` - Retrieve stored knowledge
- `serena_list_memories` - List available memories
- `serena_edit_memory <memory_file_name> <needle> <repl>` - Update memory content

## Workflow Integration
- Use Serena tools before making code changes to understand context
- Prefer symbolic tools over basic read/write operations when possible
- Store important findings in memories for future reference
- Update memories when project conventions or structure change

## Onboarding Process
- Run `serena_onboarding` for new projects to gather essential information
- Use `serena_check_onboarding_performed` to verify onboarding status