extern crate cfg_if;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
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
    pub fn new(head: usize, tail: usize) -> Cpu {
        Cpu {
            program_counter: 0,
            head: head,
            tail: tail,
        }
    }
    pub fn tick(&mut self, soup: &[Option<Instruction>]) -> bool {
        match soup[self.program_counter] {
            Some(Instruction::Nop0) => log!("nop0"),
            Some(Instruction::Nop1) => log!("nop1"),
            Some(Instruction::Divide) => {
                log!("divide");
                self.inc_counter();
                return true;
            }
            Some(inst) => log!("{:?}", inst),
            _ => log!("nothing"),
        }
        self.inc_counter();
        false
    }
    fn inc_counter(&mut self) {
        self.program_counter += 1;
        if self.program_counter > self.tail {
            self.program_counter = self.head;
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Default)]
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
        if self.cpus[self.now_cpu_idx].tick(&self.soup) {
            let cell = self.cpus[self.now_cpu_idx];
            let idx = self.first_dead_soup().unwrap();
            for n in 0..=cell.tail - cell.head {
                self.soup[idx + n] = self.soup[cell.head + n];
            }
            self.cpus.push(Cpu::new(idx, idx + cell.tail - cell.head));
        }
    }
    pub fn cpus_ptr(&self) -> *const Cpu {
        self.cpus.as_ptr()
    }
    pub fn soup_ptr(&self) -> *const Option<Instruction> {
        self.soup.as_ptr()
    }
}

impl Universe {
    fn first_dead_soup(&self) -> Option<usize> {
        for (idx, instruction) in self.soup.iter().enumerate() {
            if *instruction == None {
                return Some(idx);
            }
        }
        None
    }
}
