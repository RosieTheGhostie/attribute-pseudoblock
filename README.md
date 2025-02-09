# serdendian

A minimalistic way to serialize/deserialize endian metadata.

Provides the enum `Endian`, which indicates the intended byte order/endianness. This
information can be seamlessly processed via [`serde`](https://serde.rs).
