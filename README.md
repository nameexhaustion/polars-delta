# polars-delta

Native Delta Lake scan planning for Polars. This repository builds the `polars_delta`
Python extension and exports the `polars.io_plugin.delta.v1` planning contract.

The planner reads Delta logs and checkpoints through host-provided storage, and returns
snapshot schema, active files, partition values, statistics and deletion-vector descriptors.
Polars reads the data files and applies predicates and deletion vectors. Unsupported reader
features produce an explicit unsupported-feature result; malformed metadata is an error.

## Workspace setup

Keep this repository at `crates/polars-delta` inside a compatible Polars checkout.
The parent workspace discovers it through `crates/*`; it inherits workspace metadata,
lints and dependency versions. Its package version is independent of Polars.

The Rust package and library are `polars-delta` and `polars_delta`. The Python distribution
and import module are `polars-delta` and `polars_delta`. Enable the `python` Cargo feature
for the extension module; maturin enables it automatically.

Run Cargo checks from the parent workspace. Build wheels with maturin using this
repository's `pyproject.toml`. Follow the host checkout's build/offload instructions.

## Integration

A compatible Polars build selects the installed plugin automatically. Set
`POLARS_DELTA_PLANNER=plugin` to require it, or `POLARS_DELTA_PLANNER=deltalake` to use
the compatibility planner. Automatic mode may fall back for unsupported features;
requiring the plugin reports them directly. `use_pyarrow=True` and writes still use deltalake.

The module advertises supported contracts through `_polars_io_plugin_ids`; `_capsule`
returns the selected contract. Contract compatibility, rather than the package version
alone, determines whether a Polars build can load the plugin.

This development implementation requires a matching Polars build. The host and plugin must be released together
with compatible contracts; installing this development source alone does not add plugin
support to older Polars builds.
