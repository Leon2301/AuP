struct Computer {
    /*
    * Datenstruktur, die die Register des Rechners beschreibt
    */
    registers: Registers,

    /*
    * Ein u8-Array, das den Datenspeicher darstellt.
    * Beachten Sie, dass jedes Element des Arrays für zwei Speicherstellen steht, da ein Speicherwort ja nur vier Bits groß ist.
    * Dabei stehen die oberen (höherwertigen) vier Bits für die größere Adresse.
    * Dies entspricht etwa einem inversen Little-Endian-Format, vgl. https://de.wikipedia.org/wiki/Byte-Reihenfolge\#Little-Endian-Format
    */
    data: [u8; 128],

    /*
    * Ein u8-Array, das den Befehlsspeicher darstellt.
    */
    prog: [u8; 256],
}

struct Registers {
    pc: u8, // Befehlszähler
    fa: u8, // Flags und Akkumulator
    ix: u8, // Indexregister
}

trait Cpu {
    fn sim_cpu(&mut self) -> Vec<(usize, u8)>;
}

fn main() {
    let mut computer = Computer {
        registers: Registers {
            pc: 0,
            fa: 0,
            ix: 0,
        },
        data: [0; 128],
        prog: [0; 256],
    };
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
    if changed_cells[0].0 == 0xfe && changed_cells[0].1 == 0x0a {
        println!("cells[0] korrekt");
    }
    if changed_cells[1].0 == 0xff && changed_cells[1].1 == 0x02 {
        println!("cells[1] korrekt");
    }
}
