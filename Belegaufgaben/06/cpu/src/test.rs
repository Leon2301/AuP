#[cfg(test)]
mod tests {
    use super::*;    
    // zum testen: 'cargo new cpu --lib'
    // und dann copy pasten von der Lösung und diesem test modul in src/lib.rs

    // oder falls bereits ein cargo project erstellt ist mit src/main.rs
    // lasse die main funktion leer
    // fn main() {}

    // tests laufen: 'cargo test'
    // test alle durchlaufen, und fails überspringen: 'cargo test --no-fail-fast'

    fn new() -> Computer {
        Computer {
            registers: Registers {
                pc: 0,
                fa: 0,
                ix: 0,
            },
            data: [0; 128],
            prog: [0; 256],
        }
    }

    #[test]
    fn orig_test() {
        let mut computer = new();
        computer.prog[0] = 0b00001111; // ldi a, 15
        // => a hat den Wert 15 bzw. -1
        computer.prog[1] = 0b01001000; // mv ih, a
        // => ix enthält den Wert 240
        computer.prog[2] = 0b01000100; // mv il, a
        // => ix hat den Wert 255
        computer.prog[3] = 0b00110010; // ldi (ix), 2
        // => Adresse FF hat den Wert 2
        computer.prog[4] = 0b10100010; // add ih
        // a hat den Wert 14 bzw. -2
        computer.prog[5] = 0b01000100; // mv il, a
        // => ix hat den Wert 254
        computer.prog[6] = 0b00111010; // ldi (ix), 10
        // => Adresse FE hat den Wert A
        computer.prog[7] = 0b01111111; // halt
        // => in data[127] steht 42 (bzw. 0x2a)

        let changed_cells: Vec<(usize, u8)> = computer.sim_cpu();

        assert_eq!(changed_cells[0].0, 0xfe);
        assert_eq!(changed_cells[0].1, 0x0a);
        assert_eq!(changed_cells[1].0, 0xff);
        assert_eq!(changed_cells[1].1, 0x02);
    }

    #[test]
    fn one_hundred_test() {
        let mut computer = new();

        for i in 0..99 {
            computer.prog[i] = 0b10110000 // xor a (nop)
        }
        computer.prog[99] = 0b00010001; // ldi il, 1
        computer.prog[100] = 0b00111111; // ldi (ix), 15

        // => adr 1 does not get changed. the program has already stopped
        let changed_cells = computer.sim_cpu();

        assert_eq!(computer.registers.ix, 0b0000001);
        assert!(changed_cells.is_empty());
    }

    #[test]
    fn no_halting_test() {
        let mut computer = new();
        computer.prog[0] = 0b01011111; // jr -1 (infinite loop)

        // were testing if this runs forever or not, its supposed to
        // stop after 100 instruction calls
        // it could also fail because the pc is hitting u8::MAX

        // because of the halting problem
        // there is no way to test if an algorithm runs forever or halts.
        // thats why there are no asserts here
        let _ = computer.sim_cpu();
    }

    #[test]
    fn illegal_instruction_test() {
        let mut computer = new();
        computer.prog[0] = 0b10111111; // illegal, should halt
        computer.prog[1] = 0b00110001; // ldi (ix), 1
        let changed_cells = computer.sim_cpu();
        assert!(changed_cells.is_empty());
    }


    #[test]
    fn add_test() {
        let run_test_for = |input: u8| {
            let x = input & 0x0f;
            let y = input >> 4;            
            let mut computer = new();
            computer.prog[0] = 0b00000000 | x; // ldi a, x
            computer.prog[1] = 0b00010000 | y; // ldi il, y
            computer.prog[2] = 0b10100001; // add il
            computer.prog[3] = 0b01001100; // mv (ix), a
            computer.prog[4] = 0b01111111; // halt
            let changed_cells = computer.sim_cpu();

            let res = (x + y) % 16;
            let res_carry = (x + y) >= 16;
            let res_zero = res == 0;
            
            let flag_carry = computer.registers.fa & 0b01000000 != 0;
            let flag_zero = computer.registers.fa & 0b10000000 != 0;

            assert_eq!(flag_carry, res_carry);
            assert_eq!(flag_zero, res_zero);

            if !flag_zero {
                assert_eq!(changed_cells[0].1, res);                
            }            
        };

        for i in 0..u8::MAX {
            run_test_for(i);
        } // checks every single combination of addition
    }

    #[test]
    fn add_carry_test() {
        // algorithm for adding a u8 out of 4 bit integers;
        // adds i to j
        let run_test_for = |i: u8, j: u8| {
            let x1 = i & 0x0f;
            let y1 = i >> 4;
            let x2 = j & 0x0f;
            let y2 = j >> 4;            

            let mut computer = new();            
            computer.prog[0] = 0b00000000 | x1; // ldi a, x1
            computer.prog[1] = 0b00010000 | x2; // ldi il, x2
            computer.prog[2] = 0b10100101; // adc il            
            computer.prog[3] = 0b00010000; // ldi il, 0
            computer.prog[4] = 0b01001100; // mv (ix), a
            computer.prog[5] = 0b00000000 | y1; // ldi a, y1
            computer.prog[6] = 0b00010000 | y2; // ldi il, y2
            computer.prog[7] = 0b10100101; // adc il            
            computer.prog[8] = 0b00010001; // ldi il, 1
            computer.prog[9] = 0b01001100; // mv (ix), a

            let _ = computer.sim_cpu();
            let res = i + j;
            
            assert_eq!(computer.data[0], res);            
        };

        for i in 0..(u8::MAX - 1) {
            for j in 0..(u8::MAX - i) {
                run_test_for(i, j);
            }            
        }
    }

    #[test]
    fn binary_operations_test() {
        // op should affect il
        let run_test_for = |op: u8, res: u8| {
            let mut computer = new();
            computer.prog[0] = 0b00001100; // ldi a, 12
            computer.prog[1] = 0b00010110; // ldi il, 6
            computer.prog[2] = op;
            computer.prog[3] = 0b01001100; // mv (ix), a
            computer.prog[4] = 0b01111111; // halt
            let changed_cells = computer.sim_cpu();
            assert_eq!(changed_cells[0].1, res);
        };

        run_test_for(0b10101001, 0b0100); // and il
        run_test_for(0b10101101, 0b1110); // or il
        run_test_for(0b10110001, 0b1010); // xor il
        run_test_for(0b10110101, 0b1001); // neg il
        run_test_for(0b10111001, 0b1010); // cpl il
    }

    #[test]
    fn branch_test() {
        let mut computer = new();

        computer.prog[0] = 0b00001111; // ldi a, 15
        computer.prog[1] = 0b11000001; // b nz, 1
        computer.prog[2] = 0b00000001; // ldi a, 1
        computer.prog[3] = 0b01001100; // mv (ix), a
        computer.prog[4] = 0b11010001; // b z, 1        
        computer.prog[5] = 0b01111111; // halt
        computer.prog[6] = 0b00110001; // ldi (ix), 1
        computer.prog[7] = 0b01111111; // halt        
        let changed_cells = computer.sim_cpu();

        assert_eq!(changed_cells[0].1, 15);
    }

    #[test]
    fn jump_relative_test() {
        let mut computer = new();
        computer.prog = [0b00000001; 256]; // ldi a, 1
        
        computer.prog[0] = 0b00001111; // ldi a, 15
        computer.prog[1] = 0b01010111; // jr 7
        computer.prog[9] = 0b01010111; // jr 7
        computer.prog[17] = 0b01010111; // jr 7
        computer.prog[25] = 0b01010101; // jr 5
        computer.prog[31] = 0b01011011; // jr -4
         // this looks super wierd tbh, but its the described behavior        
        computer.prog[28] = 0b01011111; // jr -2
        computer.prog[27] = 0b01010010; // jr 2
        computer.prog[30] = 0b01010101; // jr 5               
        computer.prog[36] = 0b01001100; // mv (ix), a
        computer.prog[37] = 0b01111111; // halt

        let changed_cells = computer.sim_cpu();

        assert_eq!(changed_cells[0].1, 15);
    }
}
