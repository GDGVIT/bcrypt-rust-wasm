extern crate bcrypt_wasm;

use bcrypt_wasm::{hash, verify, DEFAULT_COST};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Bcrypt {
    cost: u32,
}

#[wasm_bindgen]
impl Bcrypt {
    pub fn new(cost: u32) -> Bcrypt {
        console_error_panic_hook::set_once();
        Bcrypt { cost }
    }

    pub fn default() -> Bcrypt {
        console_error_panic_hook::set_once();
        Bcrypt { cost: DEFAULT_COST }
    }

    #[allow(non_snake_case)]
    pub fn hashSync(&self, pass: &str) -> String {
        hash(pass, self.cost).unwrap()
    }

    #[allow(non_snake_case)]
    pub fn verifySync(&self, pass: &str, hash: &str) -> bool {
        verify(pass, hash).unwrap()
    }
}
