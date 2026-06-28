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

## [0.3.0] - 2026-06-28
- Added threaded hashing for same-size candidate files to improve scan throughput on multi-core systems.
- Kept report ordering stable by sorting hashed results before later duplicate confirmation and output.
- Added duplicate-finder coverage for larger candidate sets that exercise the parallel hashing path.

## [0.4.0] - 2026-06-28
- Added repeated `--exclude` rules so scans can skip unwanted names like `target` and relative paths like `nested/cache`.
- Applied exclusions during directory walking so ignored subtrees are never hashed or compared.
- Added CLI and duplicate-finder tests for exclusion parsing and filtered scan behavior.

## [0.5.0] - 2026-06-28
- Added summary metrics for files scanned, bytes scanned, duplicate groups, duplicate files, duplicate bytes, and elapsed time.
- Included metrics in both text and JSON output so terminal users and scripts see the same scan totals.
- Added scan-result and formatter coverage for the new reporting fields.

## [0.6.0] - 2026-06-28
- Added `--output <PATH>` so rendered text or JSON reports can be saved as manifest files.
- Created output directories automatically for export targets and rejected directory paths used as file outputs.
- Added CLI parsing and output-writer tests for saved report behavior.

## [0.7.0] - 2026-06-28
- Synced package metadata to the current shipped release and added richer Cargo package fields for distribution.
- Standardized `--version` output to `file-duplicate-finder 0.7.0` instead of printing only the raw version number.
- Added unit and binary-level tests for release-facing version behavior.
