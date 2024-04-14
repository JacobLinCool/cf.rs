# cf.rs

[CFRS[]](https://github.com/susam/cfrs) in Rust.

## Installation

```sh
cargo install cfrs
```

For library users:

```sh
cargo add cfrs
```

## Usage

### Animated GIF

```sh
cfrs out.gif '[[[[[[[[[[FS]]]]R]]RR]]RRCC]][[[[[[[[S]]]]]]]]'
```

![flower-animated.gif](./images/flower-animated.gif)

### Static Images

> All formats supported by [`image`](https://github.com/image-rs/image) crate should work.

```sh
cfrs out.bmp '[[[[[[[[[[F]]]]R]]RR]]RRCC]]'
cfrs out.jpg '[[[[[[[[[[F]]]]R]]RR]]RRCC]]'
cfrs out.png '[[[[[[[[[[F]]]]R]]RR]]RRCC]]'
cfrs out.webp '[[[[[[[[[[F]]]]R]]RR]]RRCC]]'
```

![flower.jpg](./images/flower.jpg)
