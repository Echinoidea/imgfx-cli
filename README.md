# imgmod
## CLI tool for performing logical, arithmetic, and bitwise operations on an image given a color.

----------
## Features 
- AND, OR, XOR
- ADD, SUB, MULT, DIV
- Bitwise LEFT & RIGHT

- Standard in/out image for pipelines
- Customizable left and right-hand side of operations 

## Installation

To install `ImgMod`, clone the repository and build it with `cargo`:

```bash
git clone https://github.com/yourusername/img-mod.git
cd img-mod
cargo build --release
```

## Examples
```imgmod left 1 -i ~/Pictures/ImgMod/samurai-jack.jpg | imgmod xor ff0000 --lhs b b b --rhs r r r -n | imgmod and ff0000 | imgmod left 1 > output.png```
![input](docs/images/samurai-jack.jpg)
![output](docs/images/output-samurai-jack.png)
