
# Game of life

The universe of the Game of Life is an infinite two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, alive or dead, or "populated" or "unpopulated". Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.

Any live cell with two or three live neighbours lives on to the next generation.

Any live cell with more than three live neighbours dies, as if by overpopulation.

Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules simultaneously to every cell in the seed—births and deaths occur simultaneously, and the discrete moment at which this happens is sometimes called a tick (in other words, each generation is a pure function of the preceding one). The rules continue to be applied repeatedly to create further generations.

# Project startup




#### Run Locally

Clone the project

```bash
  git clone https://github.com/dakamakat/game-of-life.git
```
#### Install Rust
Install Rust by going to the https://www.rust-lang.org/tools/install page and following the instructions. This installs a tool called ```"rustup"```, which lets you manage multiple versions of Rust. By default, it installs the latest stable Rust release, which you can use for general Rust development. Rustup installs ```rustc```, the Rust compiler, as well as ```cargo```, Rust's package manager, rust-std, Rust's standard libraries, and some helpful docs — ```rust-docs```.

#### wasm-pack
To build the package, we need an additional tool, wasm-pack. This helps compile the code to WebAssembly, as well as produce the right packaging for use in the browser. To download and install it, enter the following command into your terminal:
```bash
  cargo install wasm-pack
```

#### Go to the project directory

```bash
  cd my-project
```

#### Building the package

Now that we've got everything set up, let's build the package. We'll be using the generated code in a native ES module and in Node.js. For this purpose, we'll use the --target argument in wasm-pack build to specify what kind of WebAssembly and JavaScript is generated.

```bash
  wasm-pack build --target web
```

Serve the root directory of the project with a local web server

```bash
  example - python3 -m http.server
```

