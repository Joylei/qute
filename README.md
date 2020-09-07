# qute

QNAP device control for IT8258

## USAGE
```
qute v0.1

QNAP device control. Use AT YOUR OWN RISK!!!

USAGE: qute [OPTIONS] [COMMANDS]

OPTIONS:
  -V, --version                 Show version number
  -h, --help                    Show help message
  -v, --verbose [level:N]       Show verbose messages
  -q, --quiet                   Silence all output

COMMANDS:
  eup                           get or set Eup mode
  fan                            get or set fan speed
  power                       get or set power recovery mode
  temp                         get temperature

```
## Build
Requires cargo nightly to build the project.

Additional tools may need:
- gcc
- clang
- llvm
- `cargo +nightly install cargo-strip`

build release
```
cargo build --release
cargo strip
```