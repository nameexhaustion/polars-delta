# polars-delta-codec

Delta Lake deletion-vector validation and decoding without Polars or Arrow dependencies.

The caller provides the maximum row bound and performs storage reads. `plan_file_read` checks
offsets, serialized-size bounds and file length; validate the returned header bytes with
`validate_file_version` before fetching the frame. `decode_framed` validates frame length, CRC,
cardinality and portable Roaring data. `decode_inline` checks the declared inline size and the
same bitmap invariants. The result owns packed, least-significant-bit-first keep bits and a
logical length. Rows beyond that length are kept.

The serialized input limit is 512 MiB. The caller's row bound controls dense mask allocation;
these per-mask bounds do not limit the combined memory of concurrent readers.

This crate is independently versioned. Publish it before releasing Polars crates that depend on
its registry version. The `polars_delta` Python plugin is a separate package.
