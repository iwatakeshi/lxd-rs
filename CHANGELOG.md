# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-01-XX

### Added
- **Monorepo architecture** with workspace-based crate organization
- **OpenAPI code generator** (`codegen/`) that parses LXD's Swagger 2.0 spec
- **210+ generated types** from the official LXD API specification
- **`lxd-types`** crate with all LXD API data structures
- **`lxd-client`** crate with REST API client supporting:
  - Unix socket transport (default for local LXD)
  - HTTPS transport for remote LXD servers
- **`lxd`** umbrella crate for convenient single-import usage
- Integration tests for client functionality
- Examples demonstrating Unix socket usage

### Changed
- Complete rewrite from legacy CLI-wrapper approach to native REST API
- Simplified transport abstraction using enum-based design
- Modern async/await with Tokio runtime

### Removed
- Legacy CLI wrapper (`lxd-cli`) - LXD only runs on Linux where sockets work
- Direct LXC command execution - now using REST API exclusively

## [0.1.0] - 2019-XX-XX

### Added
- Initial release with basic LXD container management
- Container lifecycle operations (create, start, stop, delete)
- Image listing and management
- Snapshot support
- Basic server information retrieval

[Unreleased]: https://github.com/tak/lxd-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/tak/lxd-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tak/lxd-rs/releases/tag/v0.1.0
