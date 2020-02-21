<p align="center">
	<img src="https://user-images.githubusercontent.com/30529572/72455010-fb38d400-37e7-11ea-9c1e-8cdeb5f5906e.png" />
	<h2 align="center">bcrypt-wasm-rust</h2>
	<h4 align="center">Optimized bcrypt implementation in WASM, written in rust.<h4>
</p>

[![npm version](https://badge.fury.io/js/bcrypt-rust-wasm.svg)](https://badge.fury.io/js/bcrypt-rust-wasm)

----

## Functionalities
- Hash passwords
- Custom rounds for salt
- Verify hashed passwords
- Faster than the pure JS implementation

# Instructions to run

* Pre-requisites:
	-  NPM 

## Installation with NPM

```sh
npm i -s bcrypt-rust-wasm
```

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
bcrypt.verifySync("password", hash) // true
bcrypt.verifySync("password123", hash) // false
```

# Building from source
## How to build in release mode

```sh
# Builds the project and places it into the `pkg` folder.
npm run build
```

## Dependencies for building
 - [Rust](https://rustup.rs/)
 - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

# Contributors
 - @ATechnoHazard - for the implementation
 - @L04DB4L4NC3R - for the idea

<br>
<br>

 <p align="center">
	Made with :heart: by DSC VIT
</p>
