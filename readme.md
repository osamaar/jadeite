# Jadeite

Jadeite is a [high level](https://emulation.gametechwiki.com/index.php/High/Low_level_emulation) NES emulator written in rust.

The project consists of multiple crates:
* `jdasm-6502`: 6502 CPU disassembler + CLI.
* `Jadeite`: NES console emulator.
* `jadeite-ui`: UI written in rust and utilizing SDL2 rust port.

Status:
---
- [X] Complete `6502` CPU legal instruction set emulation.
- [X] Implement standalone disassembler and command line interface.
- [ ] Implement CPU & PPU debug overlay.
- [ ] Emulate `6502` CPU unofficial opcodes.
- [ ] Implement APU.

6502 Disassembler CLI:
---

- To run `jdasm-6502` binary using cargo, you need to enable its `cli` feature. Simply run, from workspace root:
```cargo r -p jdasm-6502 --features="jdasm-6502/cli"```
- **Explanation:**
By default, CLI-specific dependencies are disabled so that you can use `jdasm-6502` as a library without building the CLI binary. To pull the CLI dependencies you need to use the cargo flag: `--features="jdasm-6502/cli"` from workspace root or, if you're in the package subdirectory, simply `--features="cli"`.

- Here's the help message:
```
$ jdasm-6502 -h
jdasm-6502 0.1.0 
Osama Arafa
A 6502 binary disassembler

USAGE:
    jdasm-6502.exe [OPTIONS] <INPUT_FILE>

ARGS:
    <INPUT_FILE>    File containing binary code to be disassembled

OPTIONS:
    -h, --help                         Print help information
    -l, --length <LENGTH>              How many bytes to disassemble at most    
    -o, --output-file <OUTPUT_FILE>    Output file. Omit to print to stdout     
    -s, --offset <OFFSET>              Start position in input file [default: 0]
    -V, --version                      Print version information
```

License:
---
Jadeite is licensed under the terms of MIT license