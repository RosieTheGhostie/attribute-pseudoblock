# serdendian

A minimalistic way to serialize/deserialize endian metadata.

Provides the enum `Endian`, which indicates the intended byte order/endianness. This
information can be seamlessly processed via [`serde`](https://serde.rs).

## What serdendian Isn't

- a tool for automatically reading bytes in the right order
- a complex crate with lots of nifty features
- your mom
