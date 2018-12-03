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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Cpu {
    program_counter: usize,
    head: usize,
    tail: usize,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Instruction {
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

impl Cpu {
    pub fn tick(&mut self, soup: &Vec<Option<Instruction>>) {
        match soup[self.program_counter] {
            Some(Instruction::Nop0) => log!("nop0"),
            Some(Instruction::Nop1) => log!("nop1"),
            Some(inst) => log!("{:?}", inst),
            _ => log!("nothing"),
        }
        self.program_counter += 1;
        if self.program_counter > self.tail {
            self.program_counter = self.head;
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Universe {
    soup: Vec<Option<Instruction>>,
    cpus: Vec<Cpu>,
    now_cpu_idx: usize,
    length: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        log!("universe start!");
        let size = 10000;

        let mut soup = vec![None; size];
        let first_instruction_index = 0;
        let second_instruction_index = 1;
        let third_instruction_index = 2;
        soup[first_instruction_index] = Some(Instruction::Nop0);
        soup[second_instruction_index] = Some(Instruction::Nop1);
        soup[third_instruction_index] = Some(Instruction::Divide);

        let cpus = vec![Cpu {
            program_counter: 0,
            head: first_instruction_index,
            tail: third_instruction_index,
        }];
        Universe {
            soup: soup,
            cpus: cpus,
            now_cpu_idx: 0,
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
        self.now_cpu_idx += 1;
        if self.now_cpu_idx >= self.cpus.len() {
            self.now_cpu_idx = 0;
        }
        self.cpus[self.now_cpu_idx].tick(&self.soup);
    }
    pub fn cpus_ptr(&self) -> *const Cpu {
        self.cpus.as_ptr()
    }
    pub fn soup_ptr(&self) -> *const Option<Instruction> {
        self.soup.as_ptr()
    }
}
