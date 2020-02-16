const { Bcrypt } = require("../pkg/index");
const bcrypt = require("bcrypt");
const bcryptjs = require("bcryptjs");

// bench C bindings
console.time("Bcrypt-C-binding");
bcrypt.hash("B9qY1mIQYqmUZSeuoPD7", 10, (err, _) => {
  console.timeEnd("Bcrypt-C-binding");
});

// bench rust-WASM implementation
console.time("bcrypt-rust-wasm");
Bcrypt.new(10).hashSync("B9qY1mIQYqmUZSeuoPD7");
console.timeEnd("bcrypt-rust-wasm");

// bench pure js implementation
console.time("Bcrypt-js");
bcryptjs.hash("B9qY1mIQYqmUZSeuoPD7", 10, (err, _) => {
  console.timeEnd("Bcrypt-js");
});
