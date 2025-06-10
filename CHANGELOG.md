# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of `fish_config` function
- Support for generating Fish shell configuration from EDN input
- Cross-platform release builds for Linux, macOS, and Windows (x86_64 and ARM64)
- GitHub Actions workflows for CI and releases
- Support for Fish shell abbreviations (`:abbrs`)
- Support for aliases (`:aliases`) 
- Support for environment variables (`:env`)
- Support for PATH management (`:paths`)
- Support for custom functions (`:functions`) with multi-line support
- Support for raw Fish commands (`:fish`)
- Support for prompt configuration (`:prompt`)
- Support for custom code snippets (`:snippet/name`)
- Support for Fish greeting customization (`:fish-greeting`)
- Support for preambles (`:preambles`)
- Comprehensive documentation and examples
- Test configuration file for development

### Changed
- Updated to Rust 2024 edition
- Improved EDN pattern matching to handle both Key and Str variants
- Enhanced multi-line function body processing with proper newline handling

### Fixed
- Resolved binding modifier issues for Rust 2024 edition compatibility
- Fixed boolean value handling in prompt configuration