extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  pub fn new() -> Universe {
    log!("universe start!");
    let size = 64;

    let cells = (0..size).map(|_| {
      if js_sys::Math::random() < 0.5 {
        Cell::Alive
      } else {
        Cell::Dead
      }
    }).collect();
    Universe {
      cells,
    }
  }
  pub fn render(&self) -> String {
    self.to_string()
  }

}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in &self.cells {
            let symbol = if *cell == Cell::Dead { '◻' } else { '◼' };
            write!(f, "{}", symbol)?;
        }

        Ok(())
    }
}
