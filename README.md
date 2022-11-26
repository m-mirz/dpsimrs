# DPsimrs

The aim of this project is to try how power system simulation can be implemented in the Rust language.

## Build and test

First, open the project in the devcontainer or install all dependencies.

### rust

    $ cargo build
    $ cargo test

### python

    $ maturin build
    $ pip install ./target/wheels/dpsimrs-0.1.0-cp* --force-reinstall
    $ python3 python/tests/res_csrc_circuit.py
