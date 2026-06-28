# CHANGELOG.md
<!-- Tracks user-visible project changes by iteration. Connects to: README.md, git history. Created: 2026-06-28 -->

# Changelog

## [0.1.0] - 2026-06-28
- Added the initial Rust CLI for recursively scanning directories and identifying duplicate files.
- Added structured stderr logging, manual CLI argument parsing, and duplicate reporting output.
- Added integration tests for hashing, duplicate grouping, formatting, and CLI parsing.
- Added GitHub Actions CI for formatting checks and automated tests.

## [0.2.0] - 2026-06-28
- Added `--format text|json` so duplicate scan results can be consumed by scripts and other tools.
- Kept text output as the default format to preserve the original terminal experience.
- Added parser and formatter tests for the new JSON reporting mode.
