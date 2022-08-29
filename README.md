LLVM-RUST
============

This repository provides Rust bindings to the [SBIP-customized version of
LLVM](https://github.com/sbip-sg/llvm-project).

# Installation

## Rust

Install Rust stable by using the [rustup](https://rustup.rs/) installer as follow:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
```

## LLVM and Clang

You can download the pre-built binaries of [LLVM-SBIP](https://github.com/sbip-sg/llvm-project) for Ubuntu 20
environment or can build it from scratch (it takes 1~2 hours to build).

### Install pre-built LLVM binaries

Download the pre-built file for Ubuntu 20 environment from the [release page](https://github.com/sbip-sg/llvm-project/releases)
and extract the folder `$HOME/llvm/llvm-sbip` (or your preferred location).

After that, you need to configure some environment variables like below (copy
the code to `~/.profile`, `~/.bashrc`, `~/.zshenv`, or an equivalent file.)

```sh
# Configure the installed folder of LLVM
export LLVM_INSTALL_DIR=$HOME/llvm/llvm-sbip

# Configure system environment variables
export PATH=$LLVM_INSTALL_DIR/bin:$PATH
export LIBRARY_PATH=$LLVM_INSTALL_DIR/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=$LLVM_INSTALL_DIR/lib:$LD_LIBRARY_PATH
export DYLD_LIBRARY_PATH=$LLVM_INSTALL_DIR/lib:$DYLD_LIBRARY_PATH
```

### Build and LLVM from source code

We are currently using the branch `llvm-sbip-14` from [LLVM-SBIP](https://github.com/sbip-sg/llvm-project). Please
following the below steps to compile LLVM.


```sh
# Install prerequisite libraries
sudo apt install binutils libffi-dev

# Prepare installation folder
export LLVM_INSTALL_DIR=$HOME/llvm/llvm-sbip          # installation dir
mkdir -p $LLVM_INSTALL_DIR

# Prepare source code
mkdir -p $HOME/llvm/src                               # source code dir
cd $HOME/llvm/src
git clone https://github.com/sbip-sg/llvm-project llvm-project-sbip
export LLVM_PROJECT=$HOME/llvm/src/llvm-project-sbip
cd $LLVM_PROJECT
git checkout llvm-sbip-14

# Prepare installation directory
mkdir -p $LLVM_PROJECT/build
cd $LLVM_PROJECT/build

# Configure compilation by CMake.
# For Linux
cmake ../llvm -DCMAKE_INSTALL_PREFIX=$LLVM_INSTALL_DIR \
      -DLLVM_ENABLE_BINDINGS=OFF \
      -DLLVM_ENABLE_RTTI=ON \
      -DLLVM_BUILD_DOCS=OFF \
      -DLLVM_ENABLE_PROJECTS='clang' \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
      -Wno-dev -G Ninja
# For macOS
cmake ../llvm -DCMAKE_INSTALL_PREFIX=$LLVM_INSTALL_DIR \
      -DLLVM_ENABLE_BINDINGS=OFF \
      -DLLVM_ENABLE_RTTI=ON \
      -DLLVM_BUILD_DOCS=OFF \
      -DLLVM_ENABLE_PROJECTS='clang;compiler-rt' \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_EXPORT_COMPILE_COMMANDS=1 \
      -Wno-dev -G Ninja

# Build and install LLVM
ninja
ninja install
```

After compilation, you also need to configure the environment variables similar
to in the previous section [Install pre-built LLVM binaries](#install-pre-built-llvm-binaries).
