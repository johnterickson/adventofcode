pub struct IntCode {
    memory: Vec<isize>,
    relative_base: isize,
    pc: usize,
}

#[derive(Debug)]
pub enum CallbackAction {
    ReadInput,
    WriteOutput(isize),
}

impl IntCode {

    pub fn new(initial: &[isize]) -> IntCode {
        let mut memory : Vec<isize> = initial.to_vec();
        memory.resize(memory.len()*100, 0);
        IntCode {
            memory,
            relative_base: 0,
            pc: 0,
        }
    }

    fn get_value(&self, mode: isize, value: isize) -> isize {
        match mode {
            0 => self.memory[value as usize],
            1 => value,
            2 => self.memory[(self.relative_base + value) as usize],
            i => panic!("unimplemented mode {}", i)
        }
    }

    fn get_mut_ref(&mut self, mode: isize, value: isize) -> &mut isize {
        match mode {
            0 => &mut self.memory[value as usize],
            1 => panic!("can't write to a constant"),
            2 => &mut self.memory[(self.relative_base + value) as usize],
            i => panic!("unimplemented mode {}", i)
        }
    }
    
    pub fn run<F: FnMut(CallbackAction) -> Option<isize>>(&mut self, mut callback: F) -> () {
        loop {
            let mut instruction = self.memory[self.pc];
            let opcode = instruction % 100; instruction /= 100;
            let mode1 = instruction % 10; instruction /= 10;
            let mode2 = instruction % 10; instruction /= 10;
            let mode3 = instruction % 10; instruction /= 10;
            assert_eq!(0, instruction);

            match opcode {
                99 => {
                    return;
                },
                1 | 2 | 7 | 8 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);
                    let out = self.memory[self.pc+3];
                    let out = self.get_mut_ref(mode3, out);

                    match opcode {
                        1 => { *out = in1 + in2; }
                        2 => { *out = in1 * in2; }
                        7 => { *out = if in1 < in2 { 1 } else { 0 } }
                        8 => { *out = if in1 == in2 { 1 } else { 0 } }
                        _ => unreachable!(),
                    }

                    self.pc += 4;
                },
                3 => {
                    if let Some(input) = (callback)(CallbackAction::ReadInput) {
                        let out = self.memory[self.pc+1];
                        let out = self.get_mut_ref(mode1, out);
                        *out = input;
                        self.pc += 2;
                    } else {
                        return;
                    }
                },
                4 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    assert_eq!(0, mode2);
                    assert_eq!(0, mode3);
                    let _ = (callback)(CallbackAction::WriteOutput(in1));

                    self.pc += 2;
                },
                5 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);
                    if in1 != 0 {
                        self.pc = in2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);
                    let in2 = self.memory[self.pc+2];
                    let in2 = self.get_value(mode2, in2);

                    if in1 == 0 {
                        self.pc = in2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                9 => {
                    let in1 = self.memory[self.pc+1];
                    let in1 = self.get_value(mode1, in1);

                    self.relative_base += in1;
                    self.pc += 2;
                }

                _ => {
                    panic!("Unexpected opcode {} at {}.", opcode, self.pc);
                }
            }
        }
    }
}