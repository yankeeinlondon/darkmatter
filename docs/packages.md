# Darkmatter Packages

Darkmatter is a monorepo composed of several _packages_. Each package is intended to have a clear focus and in aggregate to facilitate the overall functional and technical goals of the Darkmatter project. On this page we'll layout the packages and what they're intention is.

## Packages

1. **dm_service** [ `Rust` ] - The **Actix** based server which provides both a REST, WS, and gRPC interface for interacting with Darkmatter
2. **dm_parser** [ `Rust` ] - The core MD-to-HTML parsing functionality
3. **dm_utils** [ `Rust` ] - Utility functions used by Rust services where the _utility_ of the features is shared across more than one other module.
4. **dm_ls** [ `Rust` ] - a Darkmatter language server which helps authors to leverage the full feature set of the parser
5. **dm_cli** [ `Rust` / `TS` ] - a Rust CLI built using the popular **Clasp** crate but also with wrappers to make it available to both Cargo and npm package managers.
6. **vite-plugin-darkmatter** [ `TS` ] - a ViteJS plugin which marshals the overall transformation pipeline as well as regulates the
7. **dm-smart-image** [ `TS` ] - detects use of the `<dm-image />` tag and ensures all images are optimized according to settings.
   > Note: this leverages the C-based Sharp library and we're using TS versus Rust for brevity/succinctness of code (most of the perf comes from Sharp library)
8. **dm-plugin** [ `TS` ] - a vs-code plugin which interacts with Darkmatter service and LSP
