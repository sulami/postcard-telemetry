# Postcard Telemetry

This library contains a logging and telemetry format that can be used
for embedded systems, particularly autonomous vehicles and other kinds
of robots, based on
[Postcard](https://github.com/jamesmunns/postcard).

The feature set and implementations are somewhat opinionated:

- This is designed to work without the use of global variables
- Telemetry data types are 32-bit

On host systems, this library can use `std` via the `std` feature,
which enables shared functionality such as log decoding.

## Building

Use `cargo hack` to build and test across both regular and `std`:

```
cargo hack --each-feature test
```
