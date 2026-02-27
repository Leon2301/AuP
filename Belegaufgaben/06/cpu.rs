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
    * ___Adressen___
    *   1   |   0   
    *   3   |   2   
    *   5   |   4  
    * ...
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

// Der Trait Cpu auf den Computer implementieren (Die Wahre Logik)
impl Cpu for Computer {
    fn sim_cpu(&mut self) -> Vec<(usize, u8)> {
        // Speichert eine Kopie des Datenspeichers vor dem Durchlaufen der Befehle. 
        let storage_before = self.data.clone();
        let mut result: Vec<(usize, u8)> = Vec::new(); 

        let mut run = true; 
        let mut counter: u16 = 0; 
        // Der CPU-Zyklus wird hier solange ausgeführt solange run == true ist. 
        while run {
            /* --- CPU-Zyklus ---
            * 1. Er liest einen Befehl aus dem Programmspeicher, 
            * dessen Adresse durch den Be­fehlszeiger PC gegeben ist.
            * 2. Er erhöht den Befehlszeiger um 1, sodass er auf den nächsten Befehl zeigt.
            * 3. Anschließend führt er den gelesenen Befehl aus und ändert 
            * dabei ggf. Register, Flags oder Speicher.
            * 4. Gehe zu Schritt 1.
            */
            // erhöhe den Zyklus counter: 
            counter += 1; 

            // Addresse aus PC holen und Befehl einlesen. 
            let befehladresse = self.registers.pc as usize; 
            let befehl = self.prog[befehladresse]; 

            // Erhöhe den Befehszeiger um 1. 
            if self.registers.pc == (self.prog.len()-1) as u8 {
                // Setzte den Befehlspointer wieder auf 0 wenn er am Ende angekommen ist. 
                self.registers.pc = 0; 
            } else {
                self.registers.pc += 1; 
            }

            // Matche den passenden Befehl
            match befehl { // Wir nutzen hier eine Bitmaske um nur die ersten 4 Bits zu bekommen. Oder eben doch Intervalle...
                0b0000_0000..=0b0011_1111 => self.ldi(befehl),
                0b0100_0000..=0b0100_1111 => self.mv(befehl), 
                0b0101_0000..=0b0101_1111 => self.jr(befehl), 
                0b0110_0000..=0b0111_1110 => run = false, // Nicht abgedeckt - soll den Prozess stoppen wie halt
                0b0111_1111 => run = false, // halt 
                0b1000_0000..=0b1001_1111 => run = false, // Nicht abgedeckt
                0b1010_0000..=0b1010_0011 => self.add(befehl),
                0b1010_0100..=0b1010_0111 => self.adc(befehl),
                0b1010_1000..=0b1010_1011 => self.and(befehl),
                0b1010_1100..=0b1010_1111 => self.or(befehl),
                0b1011_0000..=0b1011_0011 => self.xor(befehl),
                0b1011_0100..=0b1011_0111 => self.neg(befehl),
                0b1011_1000..=0b1011_1011 => self.cpl(befehl),
                0b1011_1100..=0b1011_1111 => run = false, // Nicht abgedeckt
                0b1100_0000..=0b1111_1111 => self.b(befehl), 
            }

            // Gehe zu Schritt 1, es sei denn das ist der 100 Befehlsdurchlauf
            if counter == 100 { run = false; }
            // println!("Aktueller Befehl: {:08b}", befehl);
            // println!("neuer PC: {}", self.registers.pc);
        }

        // Den Result Vektor schreiben 
        let storage_after = self.data.clone();
        // Iterriere durch alle Elemente im Datenspeicher und vergleiche Ihre Werte vorher und nachher. Schreibe gegebenenfalls den neuen Wert in den result Vektor. 
        for i in 0..256 {
            // Gerade Adressen stehen rechts im Speicher ungerade Adressen links im Speicher
            let s_b: u8 = storage_before[((i)/2) as usize];
            let s_a: u8 = storage_after[((i)/2) as usize];
            if (i) % 2 == 0 {
                if get_il(s_b) != get_il(s_a) {
                    let changes: (usize, u8) = (i as usize, get_il(s_a));
                    result.push(changes);
                }
            } else {
                if get_ih(s_b) != get_ih(s_a) {
                    let changes: (usize, u8) = ((i) as usize, get_ih(s_a));
                    result.push(changes);
                }
            }
        }
        // Da wir alle Werte der Reihe nach mit einer for-Schleife durchlaufen, sollte die Reihenfolge immer von alleine geordnet sein. 

        result
    }
}

impl Computer {
    // Tabelle 1
    // ldi - done
    fn ldi(&mut self, befehl: u8) {
        // Lädt den im Befehl enthaltenen 4-Bit-Wert val an die durch adr angegebene Stelle.
        let adr = filter_adr(befehl, 4);
        // let inhalt = self.get_adr(adr);
        let val = get_il(befehl); // get_il gibt die rechten 4 Bits wieder. 
        self.set_adr(adr, val);
    }
    // mv - done
    fn mv(&mut self, befehl: u8) { 
        // Kopiert den Wert in adr2 --> adr1
        let adr1 = filter_adr(befehl, 2);
        let adr2 = filter_adr(befehl, 0);
        let inhalt = self.get_adr(adr2);
        self.set_adr(adr1, inhalt);
    }
    // jr - done
    fn jr(&mut self, befehl: u8) {
        // Erhöht den Wert von PC um den Wertdist. Dabei ist dist im Zweierkomple­ment.
        let mut new_pc: i32 = self.registers.pc as i32; // ist vom Typ i32 da die neuen Adressen ja erstmal auch ins negative und Über u8 gehen können. 
        let dist: i32 = get_il(befehl) as i32;
        // Da wir eigentlich mit u8 rechnen, wird hier manuel das Zweierkomplement berechnet. Also i4
        if dist > 0b0000_0111 { 
            // negativer Sprung
            // bildet die positive Zahl, flattet den linker Teil und zieht das vom Pointer ab. 
            new_pc -= (!dist + 1) & 15; 
        }
        /*else if dist == 0b0000_1000 {
            // wenn dist -8 ist gibt es bei 4 Bits kein Inverses, daher Spezialfall.
            new_pc -= 8;
        } */
        else {
            // positiver Sprung
            new_pc += dist; 
        }
        // prüft ob der neue Bereich über oder unter u8 geht. 
        if new_pc < 0 { new_pc = (u8::MAX as i32) + (new_pc +1); }
        else if new_pc > u8::MAX as i32 { new_pc -= u8::MAX as i32; }
        self.registers.pc = new_pc as u8; 
    }
    // halt - wird direkt in sim_cpu ausgeführt.
    // fn halt(&self) { println!("Methode vom Computer"); 
    // b - done
    fn b(&mut self, befehl: u8) {
        // Erhöht bei erfüllter Bedingung (siehe Tabelle 3) 
        // den Wert von PC um den Wert dist. Dabei ist dist im Zweierkom­plement
        let cc: u8 = filter_adr(befehl, 4);
        // führt einen jump von pc durch wenn die Bedingung für Flags true gibt. 
        if self.cc_jump(cc) {
            // ---> selber Code wie in jr
            let mut new_pc: i32 = self.registers.pc as i32; // ist vom Typ i32 da die neuen Adressen ja erstmal auch ins negative und Über u8 gehen können. 
            let dist: i32 = get_il(befehl) as i32;
            // Da wir eigentlich mit u8 rechnen, wird hier manuel das Zweierkomplement berechnet. Also i4
            if dist > 0b0000_0111 { 
                // negativer Sprung
                // bildet die positive Zahl, flattet den linker Teil und zieht das vom Pointer ab. 
                new_pc -= (!dist + 1) & 15; 
            } else {
                // positiver Sprung
                new_pc += dist; 
            }
            // prüft ob der neue Bereich über oder unter u8 geht. 
            if new_pc < 0 { new_pc = (u8::MAX as i32) + (new_pc +1); }
            else if new_pc > u8::MAX as i32 { new_pc -= u8::MAX as i32; }
            self.registers.pc = new_pc as u8; 
        }
    }
    // add
    fn add(&mut self, befehl: u8) {
        // Addiert die Werte im Akkumulator und in adr 
        // und speichert das Ergebnis im Akkumulator.
        let a: u8 = get_il(self.registers.fa); // holt sich den Wert aus dem Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = a + inhalt_adr; 
        // Flags setzen!!
        if result > 15 { self.set_flag('C', 1); } else { self.set_flag('C', 0); } // Carry-Flag setzen
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // adc
    fn adc(&mut self, befehl: u8) {
        // Addiert die Werte im Akkumulator, in adr sowie das 
        // Übertragsflag (C) und speichert das Ergebnis im Akkumula­tor
        let a: u8 = get_il(self.registers.fa); // holt sich den Wert aus dem Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let c = (self.registers.fa & 0b0100_0000) >> 6;
        let mut result: u8 = a + inhalt_adr + c; 
        // Flags setzen Z und C
        if result > 15 { self.set_flag('C', 1); } else { self.set_flag('C', 0); } // Carry-Flag setzen
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // and - done
    fn and(&mut self, befehl: u8) {
        // Verküpft den Wert im Akkumulator und den Wert aus adr bitweise mit
        // UND und speichert das Ergebnis in Ak­kumulator.
        let a: u8 = get_il(self.registers.fa); // holt sich den Wert aus dem Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = a & inhalt_adr; 
        // Flag setzen!!
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // or
    fn or(&mut self, befehl: u8) {
        // Verküpft den Wert im Akkumulator und den Wert aus adr 
        // bitweise mit ODER und speichert das Ergebnis in Akkumulator
        let a: u8 = get_il(self.registers.fa); // holt sich den Wert aus dem Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = a | inhalt_adr; 
        // Flag setzen!
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // xor
    fn xor(&mut self, befehl: u8) {
        // Verküpft den Wert im Akkumulator und den Wert aus adr 
        // bitweise mit Exklusiv-ODER und speichert das Er­gebnis in Akkumulator
        let a: u8 = get_il(self.registers.fa); // holt sich den Wert aus dem Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = a ^ inhalt_adr; 
        // Flag setzen!!
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // neg
    fn neg(&mut self, befehl: u8) {
        // Bildet vom Wert in adr das Einerkom­plement 
        // und speichert das Ergebnis in Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = !inhalt_adr; 
        // Flag setzen!!
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }
    // cpl
    fn cpl(&mut self, befehl: u8) {
        // Bildet vom Wert in adr das Zweierkom­plement 
        // und speichert das Ergebnis im Akkumulator.
        let inhalt_adr: u8 = self.get_adr(befehl & 3); // gibt den Inhalt an der Speicheradresse von adr. 
        let mut result: u8 = !inhalt_adr +1; 
        // Flag setzen!!
        if result & 15 == 0 { self.set_flag('Z', 1); } else { self.set_flag('Z', 0); } // Zero-Flag setzen
        result = result & 15; // Damit haben wir sicher nur eine 4 Bit Zahl. 
        self.set_adr(0, result); // setzt den neuen Akkumulator Wert automatisch. 
    }

    // Tabelle 2: Kodierung der Adressen der Argumente (adr) in den Befehlen.
    // Es wird der Inhalt des entsprechenden Registers oder der Speicherstelle zurückgegeben.
    fn get_adr(&self, adr: u8) -> u8 { // gibt den Inhalt der Speicherstellen wieder. Immer nur 4 Bit lang. 
        match adr {
            0b0000_0000 => return get_il(self.registers.fa), // a - addresierung Akkumulator
            0b0000_0010 => return get_ih(self.registers.ix), // ih - Register IH
            0b0000_0001 => return get_il(self.registers.ix), // il - Register il 
            0b0000_0011 => return self.get_data(self.registers.ix), // (ix) - Inhalt der Speicherstelle, deren Adresse in IX steht (indirekte Adressierung)
            0b0000_0100..=0b1111_1111 => panic!("Das ist keine gültiges Kürzel aus Tabelle 2 {adr}"),
        };
    }
    // Setzt die Daten an die angegebene Speicherstelle
    fn set_adr(&mut self, adr: u8, val: u8) {
        if val > 15 { panic!("Daten sind größer als 4 Bits, können daher nicht in den Speicher geschrieben werden.");}
        match adr {
            0b0000_0000 => {
                let mut new_data = self.registers.fa;
                new_data &= !15; // Dadurch wird die rechte Seite gelöscht
                new_data ^= val; // das neue fa 
                self.registers.fa = new_data; // schreibe den neuen fa.
            }, // a - addresierung Akkumulator
            0b0000_0010 => {
                let mut new_data = self.registers.ix;
                new_data &= 15; // Dadurch wird die linke Seite gelöscht
                new_data ^= val << 4; // schreibe die neuen Daten auf die linke Seite; 
                self.registers.ix = new_data; // schreibe neue Daten. 
            }, // ih - Register IH
            0b0000_0001 => {
                let mut new_data = self.registers.ix;
                new_data &= !15; // Dadurch wird die rechte Seite gelöscht
                new_data ^= val; // das neue fa 
                self.registers.ix = new_data; // schreibe den neuen fa.
            }, // il - Register il 
            0b0000_0011 => {
                let adresse = self.registers.ix; // lade die Adresse
                self.set_data(adresse, val); // schreibe auf die Adresse im Datenspeicher. 
            }, // (ix) - Inhalt der Speicherstelle, deren Adresse in IX steht (indirekte Adressierung)
            0b0000_0100..=0b1111_1111 => panic!("Das ist keine gültiges Kürzel aus Tabelle 2 {adr}"),
        };
    }
    // Tabelle 3: Kodierung der Bedingungen cc in bedingten Sprüngen b cc, dist.
    fn cc_jump(&self, cc: u8) -> bool { // gibt einen bollean zurück je nach dem ob die Bedingungen fürs Springen erfüllt sind oder nicht. 
        let flags: u8 = filter_adr(self.registers.fa, 6);
        match cc {
            0b0000_0000 => { if flags & 2 == 0 {return true} else {return false} }, //  0 0 nz Das Zero-Flag ist nicht gesetzt (Z= 0)
            0b0000_0001 => { if flags & 2 == 2 {return true} else {return false} }, //  0 1 z Das Zero-Flag ist gesetzt (Z= 1)
            0b0000_0010 => { if flags & 1 == 0 {return true} else {return false} }, //  1 0 nc Das Übertragsflag ist nicht gesetzt (C= 0)
            0b0000_0011 => { if flags & 1 == 1 {return true} else {return false} }, //  1 1 c Das Übertragsflag ist gesetzt (C= 1)
            0b0000_0100..=0b1111_1111 => panic!("Das ist keine gültiges Kürzel aus Tabelle 3 {cc}"),
        };
    }
    // Datenladen - Sozusagen der Datenbus in die eine Richtung
    fn get_data(&self, adresse: u8) -> u8 {
        // Gerade Adressen stehen rechts im Speicher ungerade Adressen links im Speicher
        let double_data: u8 = self.data[(adresse/2) as usize];
        if adresse % 2 == 0 {
            get_il(double_data)
        } else {
            get_ih(double_data)
        }
    }
    // Datenschreiben - Sozusagen der Datenbus in die eine Richtung - Data darf max 4 Bits haben.
    fn set_data(&mut self, adresse: u8, data: u8) {
        // Fehlerhafte Daten Abfangen: 
        if data > 15 { panic!("Daten sind größer als 4 Bits, können daher nicht in den Speicher geschrieben werden.");}
        // Gerade Adressen stehen rechts im Speicher ungerade Adressen links im Speicher
        let mut double_data: u8 = self.data[(adresse/2) as usize];
        if adresse % 2 == 0 {
            double_data &= !15; // Dadurch wird die rechte Seite gelöscht
            double_data ^= data; // schreibe die neuen Daten auf die rechte Seite; 
            self.data[(adresse/2) as usize] = double_data;
        } else {
            double_data &= 15; // Dadurch wird die linke Seite gelöscht
            double_data ^= data << 4; // schreibe die neuen Daten auf die linke Seite; 
            self.data[(adresse/2) as usize] = double_data;
        }
    }
    // Flage setzen. flag muss Z oder C sein und val 1 oder 0
    fn set_flag(&mut self, flag: char, val: u8) {
        let mut new_data = self.registers.fa;
        match flag {
            'Z' => {
                new_data &= 0b0111_1111; // Dadurch wird Bit mit Index 7 gelöscht.
                new_data ^= val << 7; // das neue fa 
            },
            'C' => {
                new_data &= 0b1011_1111; // Dadurch wird Bit mit Index 7 gelöscht.
                new_data ^= val << 6; // das neue fa 
            },
            _ => panic!("Das ist keine gültige Flage! (set_flag)"),
        };
        self.registers.fa = new_data; // schreibe den neuen fa.
    }
}

// --- Hilfsfunktionen ---

// Gitbt IH, die linke Addresse in ix. Diese steht dann an den hinteren vier Stellen. 
fn get_ih(adr: u8) -> u8 {
    let mut x = adr; 
    x &= !15; // Bei 15 sind die niedriger Wertigen Bits 1 und mit ! werden die Bits geswitched. 
    x = x >> 4;
    x
}
// Gitbt IL, die rechte Addresse in ix. Setzt die ersten vier Bits auf 0. 
fn get_il(adr: u8) -> u8 {
    let mut x = adr; 
    x &= 15;
    x
}
// filtert die Adresse heraus welche an bit n beginnt und schiebt sie nach rechts
fn filter_adr(adr: u8, n: usize) -> u8 { // Der Rückgabewert hat immer nur Informationen in Bit 1 und 0 der rest ist alles 0. 
    let mut x = adr; 
    x = x >> n;
    x &= 0b0000_0011; 
    x
}
