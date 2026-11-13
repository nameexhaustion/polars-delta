# polars-delta

Native Delta Lake scan planning for Polars. This repository builds the `polars_delta`
Python extension and exports the `polars.io_plugin.delta.v1` planning contract. It also owns
the standalone `polars-delta-codec` Rust package in `codec/`.

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
repository's `pyproject.toml`. Follow the host checkout's build/offload instructions. After
`maturin sdist`, run the host's `tools/normalize-sdist-workspace.py` with Python 3.11 or later
on the generated archives before testing or publishing them; release workflows do this automatically.

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

## Deletion-vector codec

`polars-delta-codec` validates Delta deletion-vector framing and decodes portable Roaring data
into packed keep bits. It depends only on `crc32fast`, not on Polars, Arrow, Python or this
planner. The host links it statically for native reads with either planner; storage credentials,
lazy range reads, pruning, masks and reader scheduling remain host responsibilities. The Python
planning plugin remains optional at runtime.

The compatible Polars source checkout requires this repository, at its pinned revision, before
Cargo metadata or builds. Host and plugin source distributions include the codec dependency.
The codec has independent package metadata and must be published to the Rust registry before
host crates depending on its version. Publishing the Python plugin does not publish the codec.
