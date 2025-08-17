# Changelog

All notable changes to this project will be documented in this file.

## [2.2.0] - 2025-08-17
- Added 3 flags for writing result output into a file
  - `--output-file` (-o)
  - `--append` (-a)
  - `--enumerate`

## [2.1.4] - 2025-07-14
### Added
- Included a `CHANGELOG.md` file to keep track of changes
- Added an `Installation` section in the `README.md` file

## [2.1.3] - 2025-06-26
### Changed
- Renamed the cache file to a hidden dot file: `.info.json`

## [2.1.2] - 2025-06-02
### Fixed
- Fixed logic error in `--cache (-c)` flag:
  - When a valid cache was provided, the early-exit condition was missing.
  - Added the correct condition to exit early if the cache is valid.

## [2.1.1] - 2025-06-02
### Improved
- Optimized `search()` to be asynchronous
  - Result: approximately 50% performance improvement in search times

## [2.1.0] - 2025-03-27
### Changed
- CLI flag change: `-d` is now reserved for another option
  - To specify depth, users must use `--depth` explicitly

## [2.0.2] - 2025-03-27
### Fixed
- Adjusted the position of logging messages for clearer output

## [2.0.1] - 2025-03-27
### Fixed
- Minor correction to logging message content/formatting

## [2.0.0] - 2025-03-27
### Added
- Major reimplementation of the tool
- Now supports regex-based queries with optional flags
