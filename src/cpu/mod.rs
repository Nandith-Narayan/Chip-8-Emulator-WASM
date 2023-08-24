//use crate::alert;
use crate::cpu::Instruction::{AddToRegister, ClearScreen, Display, Jump, SetIndexRegister, SetRegister, SkipIfRegContains, NOP, SkipIfRegDoesNotContains, SkipIfEqual, SkipIfNotEqual, Return, Call, MathOp};

const VF: usize = 15;
pub struct CPU{
    pub regs: [u8; 16],
    pub index_reg: usize,
    pub stack: [usize; 16],
    pub sp: usize,
    pub pc: usize,
    pub memory: Vec<u8>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: Vec<bool>,
}

#[derive(Debug)]
enum Instruction{
    NOP,
    ClearScreen,
    Jump{addr:usize},
    SetRegister{register:usize, value:u8},
    AddToRegister{register:usize, value:u8},
    SetIndexRegister{addr: usize},
    Display{x_reg:usize, y_reg:usize, n:usize},
    SkipIfRegContains{register:usize, value:u8},
    SkipIfRegDoesNotContains{register:usize, value:u8},
    SkipIfEqual{x_reg:usize, y_reg:usize},
    SkipIfNotEqual{x_reg:usize, y_reg:usize},
    Return,
    Call{addr:usize},
    MathOp{x_reg:usize, y_reg:usize, op:u8}
}

static FONT_ARRAY: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

pub fn init() ->CPU{

    let mut memory = vec![0; 4096];

    memory[..(FONT_ARRAY.len())].copy_from_slice(&FONT_ARRAY[..]);


    return CPU{
        regs: [0; 16],
        index_reg: 0,
        stack: [0; 16],
        sp: 0,
        pc: 512,
        memory,
        delay_timer: 0,
        sound_timer: 0,
        display: vec![false; 64*32],
    };
}

impl CPU{
    pub fn run_cycle(&mut self){
        if self.pc >= 4095{
            return;
        }

        // Fetch
        let instruction = (((self.memory[self.pc] as usize) << 8) + (self.memory[self.pc+1] as usize)) & 0x0FFFF;


        // Decode
        let decoded_instruction = match instruction {
            0x00E0 => ClearScreen, // Clear Screen 0x00E0
            0x00EE => Return, // Return 0x00EE
            a if a & 0x0F000 == 0x01000 => Jump {addr: a&0x0FFF}, // Jump 0x1NNN
            a if a & 0x0F000 == 0x02000 => Call {addr: a&0x0FFF}, // Call 0x2NNN
            a if a & 0x0F000 == 0x03000 => SkipIfRegContains {register: (a&0x0F00)>>8, value: (a & 0x0FF) as u8}, // Skip Next Instruction if register contains value 0x3XNN
            a if a & 0x0F000 == 0x04000 => SkipIfRegDoesNotContains {register: (a&0x0F00)>>8, value: (a & 0x0FF) as u8}, // Skip Next Instruction if register does not contains value 0x4XNN
            a if a & 0x0F000 == 0x05000 => SkipIfEqual {x_reg: (a&0x0F00)>>8, y_reg: (a&0x00F0)>>4}, // Skip Next Instruction if registers are equal 0x5XY0
            a if a & 0x0F000 == 0x06000 => SetRegister {register: (a&0x0F00)>>8, value: (a & 0x0FF) as u8}, // Set Register 0x6XNN
            a if a & 0x0F000 == 0x07000 => AddToRegister {register: (a&0x0F00)>>8, value: (a & 0x0FF) as u8}, // Add to Register 0x7XNN
            a if a & 0x0F000 == 0x08000 => MathOp {x_reg: (a&0x0F00)>>8, y_reg: (a&0x00F0)>>4, op: (a & 0x00F) as u8 }, // Math Operations 0x8XYN
            a if a & 0x0F000 == 0x09000 => SkipIfNotEqual {x_reg: (a&0x0F00)>>8, y_reg: (a&0x00F0)>>4}, // Skip Next Instruction if registers are not equal 0x9XY0
            a if a & 0x0F000 == 0x0A000 => SetIndexRegister {addr: a&0x0FFF}, // Set Index Register 0xANNN
            a if a & 0x0F000 == 0x0D000 => Display {x_reg: (a&0x0F00)>>8, y_reg: (a&0x00F0)>>4, n: a & 0x00F}, // Display 0xDXYN

            _ => NOP // No Operation
        };
        //alert(format!("{:#04x}, {:?}", instruction, decoded_instruction).as_str());
        // Execute
        match decoded_instruction{
            NOP => {
                self.pc += 2;
            },
            ClearScreen => {
                //self.display = vec![false; 64*32];
                self.pc += 2;
            },
            Jump {addr} => {
                self.pc = addr;
            },
            SetRegister {register, value} => {
                self.regs[register] = value;
                self.pc += 2;
            },
            AddToRegister {register,value} => {
                self.regs[register] = self.regs[register].wrapping_add(value);
                self.pc += 2;
            },
            SetIndexRegister {addr} => {
                self.index_reg = addr;
                self.pc += 2;
            },
            Display {x_reg, y_reg, n} => {
                //alert("hi");

                let mut y = (self.regs[y_reg] % 32) as usize;
                self.regs[VF] = 0;

                for row in 0..n{
                    let mut sprite_data = self.memory[self.index_reg+row];
                    let mut x = (self.regs[x_reg] % 64) as usize;
                    for _ in 0..8{
                        let new_pixel = (sprite_data &0x080) > 0;
                        sprite_data = sprite_data << 1;
                        let old_pixel = self.display[x+y*64];
                        if x < 64 && y < 32{
                            if new_pixel && old_pixel{
                                self.regs[VF] = 1;
                                self.display[x+y*64] = false;
                            }else if new_pixel && !old_pixel{
                                self.display[x+y*64] = true;
                            }
                        }
                        x+=1;
                    }
                    y+=1;
                }
                self.pc += 2;
            },
            SkipIfRegContains {register, value} => {
                if self.regs[register] == value{
                    self.pc += 2;
                }
                self.pc += 2;
            },
            SkipIfRegDoesNotContains {register, value} => {
                if self.regs[register] != value{
                    self.pc += 2;
                }
                self.pc += 2;
            },
            SkipIfEqual {x_reg, y_reg} => {
                if self.regs[x_reg] == self.regs[y_reg]{
                    self.pc += 2;
                }
                self.pc += 2;
            },
            SkipIfNotEqual {x_reg, y_reg} => {
                if self.regs[x_reg] != self.regs[y_reg]{
                    self.pc += 2;
                }
                self.pc += 2;
            },
            Call{addr} => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = addr;
            },
            Return => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
                self.pc += 2;
            }
            MathOp {x_reg, y_reg, op} => {
                match op{
                    0 => { // 0: Set Vx = Vy
                        self.regs[x_reg] = self.regs[y_reg];
                    },
                    1 => { // 1: Vx = Vx OR Vy
                        self.regs[x_reg] = self.regs[x_reg] | self.regs[y_reg];
                    },
                    2 => { // 2: Vx = Vx AND Vy
                        self.regs[x_reg] = self.regs[x_reg] & self.regs[y_reg];
                    },
                    3 => { // 3: Vx = Vx XOR Vy
                        self.regs[x_reg] = self.regs[x_reg] ^ self.regs[y_reg];
                    },
                    4 => { // 4: Vx = Vx + Vy
                        let result:u16 = (self.regs[x_reg] as u16) + (self.regs[y_reg] as u16);
                        if result > 255 {
                            self.regs[VF] = 1;
                        }else{
                            self.regs[VF] = 0;
                        }
                        self.regs[x_reg] = (result & 0x0FF) as u8;
                    },
                    5 => { // 5: Vx = Vx - Vy
                        let a = self.regs[x_reg];
                        let b = self.regs[y_reg];

                        self.regs[VF] = 1;
                        if b > a{
                            self.regs[VF] = 0;
                        }

                        self.regs[x_reg] = a.wrapping_sub(b);
                    },
                    6 => { // 6: Vx = Vy >> 1
                        self.regs[x_reg] = self.regs[y_reg];

                        self.regs[VF] = self.regs[x_reg] & 0x01;

                        self.regs[x_reg] = self.regs[x_reg] >> 1;
                    },
                    7 => { // 7: Vx = Vy - Vx
                        let a = self.regs[y_reg];
                        let b = self.regs[x_reg];

                        self.regs[VF] = 1;
                        if b > a{
                            self.regs[VF] = 0;
                        }

                        self.regs[x_reg] = a.wrapping_sub(b);
                    },
                    0xE => { // 6: Vx = Vy << 1
                        self.regs[x_reg] = self.regs[y_reg];

                        self.regs[VF] = (self.regs[x_reg] & 0x080) >> 7;

                        self.regs[x_reg] = self.regs[x_reg] << 1;
                    },
                    _ => {}
                };
                self.pc += 2;
            }
        };

    }
}