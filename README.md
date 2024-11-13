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

To install `imgmod`, clone the repository and build it with `cargo`:

```bash
git clone https://github.com/Echinoidea/img-mod.git imgmod
cd imgmod
cargo build --release
```

## Usage
```
Usage: imgmod [OPTIONS] <COMMAND>

Commands:
  or     
  and    
  xor    
  left   
  right  
  add    
  sub    
  mult   
  div    
  help   Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>          path/to/input/image
      --output <OUTPUT>        path/to/output/image [default: .]
      --lhs <LHS>...           Specify the left hand side operands for the function. E.g. --lhs b g r
      --rhs <RHS>...           Specify the right hand side operands for the function. E.g. --rhs b r b
  -b, --bit-shift <BIT_SHIFT>  If function is 'left' or 'right', how many bits to shift by
  -n, --negate                 Negate the logical operator
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples
```imgmod left 1 -i samurai-jack.jpg | imgmod xor ff0000 --lhs b b b --rhs r r r -n | imgmod and ff0000 | imgmod left 1 > output.png```
![input](docs/images/samurai-jack.jpg)
![output](docs/images/output-samurai-jack.png)

```imgmod -i flcl.png left 4 | imgmod and f788c72```
![input](docs/images/flcl.png)
![output](docs/images/output-flcl.png)

```imgmod -i ultramurder.png div ff0000 --lhs g g g --rhs b r b```
![input](docs/images/ultramurder.png)
![output](docs/images/output-ultrakill-isolated.png)
