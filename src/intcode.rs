pub struct IntCode {
    mem: Vec<i64>,
    pos: usize,
    rel: usize,
}

impl IntCode {
    pub fn new(program: &[i64]) -> IntCode {
        IntCode {
            mem: program.to_vec(),
            pos: 0,
            rel: 0,
        }
    }

    pub fn run(self: &mut Self, mut input: impl FnMut() -> i64, mut output: impl FnMut(i64)) {
        while self.step(&mut input, &mut output) != 99 {}
    }

    fn cell_abs(self: &mut Self, addr: usize) -> &mut i64 {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        &mut self.mem[addr]
    }

    fn cell_rel(self: &mut Self, i: usize) -> &mut i64 {
        self.cell_abs(self.pos + i)
    }

    fn mode(self: &mut Self, i: usize) -> i64 {
        (*self.cell_rel(0) / 10i64.pow(i as u32 + 1)) % 10
    }

    fn read(self: &mut Self, i: usize) -> i64 {
        let addr = *self.cell_rel(i);
        match self.mode(i) {
            0 => *self.cell_abs(addr as usize),
            1 => addr,
            2 => *self.cell_abs((self.rel as i64 + addr) as usize),
            _ => panic!("unrecognized mode!"),
        }
    }

    fn write(self: &mut Self, i: usize, val: i64) {
        let addr = *self.cell_rel(i);
        let offset = match self.mode(i) {
            0 => 0,
            1 => panic!("write address cannot be in immediate mode!"),
            2 => self.rel as i64,
            _ => panic!("unrecognized mode!"),
        };
        *self.cell_abs((addr + offset) as usize) = val;
    }

    fn step(self: &mut Self, input: &mut impl FnMut() -> i64, output: &mut impl FnMut(i64)) -> i64 {
        let opcode = self.mem[self.pos] % 100;

        match opcode {
            1 => {
                let arg1 = self.read(1);
                let arg2 = self.read(2);
                self.write(3, arg1 + arg2);
                self.pos += 4;
            }
            2 => {
                let arg1 = self.read(1);
                let arg2 = self.read(2);
                self.write(3, arg1 * arg2);
                self.pos += 4;
            }
            3 => {
                self.write(1, input());
                self.pos += 2;
            }
            4 => {
                output(self.read(1));
                self.pos += 2;
            }
            5 => {
                if self.read(1) != 0 {
                    self.pos = self.read(2) as usize;
                } else {
                    self.pos += 3;
                };
            }
            6 => {
                if self.read(1) == 0 {
                    self.pos = self.read(2) as usize;
                } else {
                    self.pos += 3;
                }
            }
            7 => {
                let arg1 = self.read(1);
                let arg2 = self.read(2);
                self.write(3, if arg1 < arg2 { 1 } else { 0 });
                self.pos += 4;
            }
            8 => {
                let arg1 = self.read(1);
                let arg2 = self.read(2);
                self.write(3, if arg1 == arg2 { 1 } else { 0 });
                self.pos += 4;
            }
            9 => {
                self.rel = (self.rel as i64 + self.read(1)) as usize;
                self.pos += 2;
            }
            99 => {}
            _ => panic!("unrecognized opcode {}!", opcode),
        }

        opcode
    }
}
