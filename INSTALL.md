# Installation procedure


## Using NIX

This is a research project, and therefore it is unlikely to be working as-is on
your machine. However, in order to ease the installation process, we provide
a `nix-shell` environment that should work on any machine with `nix` installed.

If you already have `nix` installed, you can simply run in the root of the
repository:

```bash
nix-shell
```

And you will be in a shell with all dependencies installed. In particular, you
will have access to `cvc5`, `z3`, `alt-ergo`, and `mona`, which are the solvers
that we use in our experiments.

To install `nix`, you can follow the instructions at <https://nixos.org/download.html>.


### Compiling 

The main part of the project lies in the directory `simplified_transducer`.
To build it, you need to use `cargo`, the Rust package manager. You can build
the project by running:

```bash
cargo build --release
```

This will create the binary `simplified_transducer/target/release/simplified_transducer`.

### Installing the executable

There is, up to now, no installation script. You can however copy the binary
to a directory in your `PATH`:

```
cp simplified_transducer/target/release/simplified_transducer ~/.local/bin/
```

However, keep in mind that the executable will try to find the solvers `cvc5`,
`z3`, `alt-ergo`, and `mona`, so that you need to have them in your `PATH` as
well (which can be achieved by running the `nix-shell` command described
above).

## Without Nix

If you do not have `nix` installed, you can still install
the dependencies manually. You will need to install the following dependencies:

- `cvc5`
- `z3`
- `alt-ergo`
- `mona`
- `rust` (with `cargo`)

Note that [mona](https://www.brics.dk/mona/) is not available in the package
manager of most distributions, so you will need to compile it from source. The
source code is available as a [git
submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules) of this git
repository.


### Compiling

The compilation process is the same as with `nix`:

```bash
cargo build --release
```

### Installing the executable

There is no installation procedure yet.
