# wasm-profiler

**WORK IN PROGRESS**

Utilities for profiling WebAssembly binaries.

## Specification

WebAssembly profiling should consists of a CSV output with two colums: function index and duration (in microseconds).
The same function index can be appear multiple times, in which case its durations must be summed first.

## Maintainers

* Alex Beregszaszi [@axic]

## License

[Apache 2.0](LICENSE).

[@axic]: https://github.com/axic
