pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// pub fields for debugging :(
pub struct Emulator {
    pub pc: u16,
    pub ram: [u8; RAM_SIZE],
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub v_reg: [u8; NUM_REGS],
    pub i_reg: u16,
    pub sp: u16,
    pub stack: [u16; STACK_SIZE],
    pub keys: [bool; NUM_KEYS],
    pub dt: u8,
    pub st: u8,
    pub op: u16, // debugging
}

#[allow(dead_code)]
impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
            op: 0,
        };

        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        self.op = 0;
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        let lin = x + y * SCREEN_WIDTH;
        self.screen[lin]
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.op = op;
        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xf000) >> 12;
        let d2 = (op & 0x0f00) >> 8;
        let d3 = (op & 0x00f0) >> 4;
        let d4 = op & 0x000f;

        let nnn = op & 0xfff;
        let nn = (op & 0xff) as u8;
        let x = d2 as usize;
        let y = d3 as usize;
        let n = d4;

        match (d1, d2, d3, d4) {
            // NOP
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // JMP NNN
            (1, _, _, _) => {
                self.pc = nnn;
            }
            // VX = NN
            (6, _, _, _) => {
                self.v_reg[x] = nn;
            }
            // VX += NN
            (7, _, _, _) => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            // I = NNN
            (0xa, _, _, _) => {
                self.i_reg = nnn;
            }
            // DRAW X Y N
            (0xd, _, _, _) => {
                let x_coord = self.v_reg[x] as u16;
                let y_coord = self.v_reg[y] as u16;
                let num_rows = n as u16;
                let mut flipped = false;

                for y_line in 0..num_rows {
                    let addr = (self.i_reg + y_line) as usize;
                    let pixels = self.ram[addr];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x_pos = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y_pos = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                            let lin = x_pos + y_pos * SCREEN_WIDTH;

                            flipped |= self.screen[lin];
                            self.screen[lin] ^= true;
                        }
                    }
                }

                self.v_reg[0xf] = flipped as u8;
            }
            (_, _, _, _) => panic!("invalid op: {:#04x}", op),
        }
    }
}
