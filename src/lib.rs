extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;
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

impl Cell {
    pub fn tick(&self) -> Option<Cell> {
        if *self == Cell::Dead {
            return None
        }
        if js_sys::Math::random() < 0.5 {
            Some(Cell::Alive)
        } else {
            None
        }
    }
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

        let mut cells = vec![Cell::Dead; size];
        cells[0] = Cell::Alive;
        Universe { cells }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) {
        let mut new_cells = vec![];
        let cells = self.cells.clone();
        for cell in &cells {
            match cell.tick() {
                None => continue,
                Some(cell) => new_cells.push(cell),
            }
            let idx = self.first_dead_cell().unwrap();
            self.cells[idx] = *cell;
        }
    }
}

impl Universe {
    fn first_dead_cell(&self) -> Option<usize> {
        for (idx, cell) in self.cells.iter().enumerate() {
            if *cell == Cell::Dead {
                return Some(idx);
            }
        }
        return None;
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
