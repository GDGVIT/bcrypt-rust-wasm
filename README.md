# bcrypt-rust-wasm
Optimized bcrypt implementation in WASM, written in rust.

[![npm version](https://badge.fury.io/js/bcrypt-rust-wasm.svg)](https://badge.fury.io/js/bcrypt-rust-wasm)

## How to install

```sh
npm i -s bcrypt-rust-wasm
```


## How to build in release mode

```sh
# Builds the project and places it into the `pkg` folder.
npm run build
```

## Dependencies for building
 - [Rust](https://rustup.rs/)
 - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# Usage
This library is currently compatible with NodeJS and supports the synchronous bindings only.

## Importing
```js
const { Bcrypt } = require('bcrypt-rust-wasm');
...
```

## Obtaining an instance
```js
// Get a bcrypt instance with a default number of seed rounds
const bcryptDefault = Bcrypt.default();

// Get a bcrypt instance with a specified number of seed rounds
const bcrypt = Bcrypt.new(10);
```

## Hashing
```js
const hash = bcrypt.hashSync('password'); 
// hash: "$2b$04$fsiGFAMtgNJ8YIszhoxusObEgoLF.faqMIKXiRDp5GZbzFWzebgcu"
```

## Verifying a hash
```js
bcrypt.verify("password", hash) // true
bcrypt.verify("password123", hash) // false
```