# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-12-11

### Added

- Baked in build info for debugging purposes (view with the `--debug-info` flag)
- Proper logging and more error reporting
- Support for TOML configuration files
- Syntax verification for config files
- Glob support for input paths
- Support for parsing classes delimited by `.`, in config files

### Changed

- Moved from `java-spaghetti` to `java-oxide` (literally just a name change)
- Renamed almost all of the configuration fields to better suit the new TOML format
- Now only looks at the classfiles packaged in a JAR, instead of every single file.
- Generates bindings using absolute paths (`crate::...`) instead of relative paths (`super::super::...`)

### Removed

- Support for YAML configuration files

[unreleased]: https://github.com/OxidizeMC/java-oxide/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/OxidizeMC/java-oxide/releases/tag/v0.1.0
