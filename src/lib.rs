extern crate cfg_if;
extern crate js_sys;
#[macro_use]
extern crate serde_derive;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

enum Instruction {
    Adr,
    Adrb,
    Adrf,
    Call,
    DecC,
    Divide,
    IfCz,
    IncA,
    IncB,
    IncC,
    Jmp,
    JumpB,
    Mal,
    MovAb,
    MovCd,
    MovIab,
    Nop0,
    Nop1,
    Or1,
    PopAx,
    PopBx,
    PopCx,
    PopDx,
    PushAx,
    PushBx,
    PushCx,
    PushDx,
    Ret,
    Sh1,
    SubAb,
    SubAc,
    Zero,
}

impl Cell {
    pub fn tick(&self) -> Option<Cell> {
        if *self == Cell::Dead {
            return None;
        }
        if js_sys::Math::random() < 0.5 {
            Some(Cell::Alive)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Universe {
    cells: Vec<Cell>,
    now_cell_idx: usize,
    length: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        log!("universe start!");
        let size = 10000;

        let mut cells = vec![Cell::Dead; size];
        cells[0] = Cell::Alive;
        Universe {
            cells: cells,
            now_cell_idx: 0,
            length: size,
        }
    }
    pub fn length(&self) -> usize {
        self.length
    }
    pub fn render_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn tick(&mut self) {
        self.now_cell_idx += 1;
        if self.now_cell_idx >= self.length {
            self.now_cell_idx = 0;
        }
        let new_cell = match self.cells[self.now_cell_idx].tick() {
            None => return,
            Some(cell) => cell,
        };
        let idx = match self.first_dead_cell() {
            None => return,
            Some(idx) => idx,
        };
        self.cells[idx] = new_cell;
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
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
