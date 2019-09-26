# wasm-profiler

**WORK IN PROGRESS**

Utilities for profiling WebAssembly binaries.

## Installation

```
cargo install wasm-profiler
```

## Usage

```
wasm-profiler <profile.wasmprof> [module.wasm]
```

## Specification

WebAssembly profiling should consists of a CSV output with two colums: function index and duration (in microseconds).
The same function index can be appear multiple times, in which case its durations must be summed first.

### Example

As an example:
```csv
func_index,duration
1,33
55,1234
1,22
1,11
2,44
3,55
4,66
```

This means that functions 1, 2, 3, 4 and 55 ran for 66us, 44us, 55us, 66us and 1234us, respectively.

The tool would output something like (assuming the symbols `memcpy`, `memset`, `strcmp` and `strcpy` map to function indexes 1, 2, 3 and 4, respectively):
```
Total time taken 1465us
Function <index:55> took 1234us (84%)
Function memcpy took 66us (4%)
Function strcpy took 66us (4%)
Function strcmp took 55us (3%)
Function memset took 44us (3%)
```

## Library

This create also supplies a library called `wasmprofiler`. It supports loading profiling data programatically or from a CSV file.

## Maintainers

* Alex Beregszaszi [@axic]

## License

[Apache 2.0](LICENSE).

[@axic]: https://github.com/axic
