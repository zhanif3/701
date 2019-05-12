### Compilation
To compile Mori, the user will need a copy of the source repository and a Rust compiler. Mori has been built on Mac OSX
and Ubuntu Linux 18.04. To compile Mori on Mac OSX, run the following commands in your terminal:

```
git clone https://github.com/zhanif3/701
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cd 701
cargo build --release
```

To compile Mori on an Ubuntu machine, run the following commands in your terminal:

```
git clone https://github.com/zhanif3/701
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cd 701
sudo apt-get update
sudo apt-get install build-essential
cargo build --release
```

The result of these commands is functionally the same - a series of compiled binaries will be created within the
target/release directory. After this has been done, an example of Mori can be executed by running:

```
target/release/all_in_one other_data/10000_dna TTAGAGAAA
```

Which will run the `all_in_one` binary over a specifed file, with `TTAGAGAAA` as the search string. The expected output
is below:

```
5, 61, 208, 7, "TTAGAGAAA", "other_data/10000_dna"
```

Serialization functionality can be demonstrated as well:

```
target/release/generator other_data/10000_dna
```

This command will write several serialized files to the current directory, which will be read by the search client
to perform searches on a pre-computed index.

```
target/release/client TTAGAGAAA
```

A small index is shipped with the Github repository, to allow the client the ability to run without running the generator.
