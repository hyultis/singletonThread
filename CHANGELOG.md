# Changelog

All notable changes to this project will be documented in this file.  
This project adheres to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and follows [Semantic Versioning](https://semver.org/).

---

## [2.1.0]

### Added
- Added `drop` feature to disable automatic dropping of threads on exit.
- Added `wait` method to wait for a thread to finish.

## [2.0.0]

### Added
- Moved priority stuff into optional feature `thread-priority`.

### Changed
- **[Breaking]** Refactored most of the method names.
- Refactored internal thread management for better performance and maintainability.
- Updated dependency.
- Updated to Rust 2024 edition.
