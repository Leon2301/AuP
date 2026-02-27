// Übung 03 AuP - Leon Mantey
// Der Kompiler ist mein Freund, da er sehr genaue Hinweise dafür gibt was falsch ist. 
// Self ist der Datentyp
// self ist der wirklich konkrete "Inhalt" der gerade übergeben wurde.

trait print_type {
    fn print_type(self) -> Self;
}

// Aufgabe 3: Trait für große Zahlen
trait Large {
    fn is_large(self) -> bool; 
}

impl Large for f32{
    fn is_large(self) -> bool {
        // mit Min und Max kann man den Wertebereich eines Typs herausfinden
        // Hier wird direkt ein bool zurückgegeben, auch ohne if
        (f32::MIN + f32::MAX)/2.0 <= self
    }
}

impl Large for f64{
    fn is_large(self) -> bool {
        (f64::MIN + f64::MAX)/2.0 <= self
    }
}

// Aufgabe 4: Power
trait Power {
    fn power(self, exp: u32) -> Self;
}

impl Power for f64 {
    fn power(self, exp: u32) -> Self {
        if exp == 0 {
            1.0
        } else {
            self * self.power(exp - 1) // bei dem rekursiven Aufruf wird self nicht verändert, 
                                      //sondern nur exp wird dekrementiert
        }
    }
}


fn main() {
    println!("Hello, world!");
    let a: f32 = 10.0;
    println!("is {} large? {}", a, a.is_large());
    println!("5 hoch 3 ist {}", 5_f64.power(3)); 
}

// Aufgabe 1: Prüfen, ob eine Zahl ein Vielfaches einer anderen ist
fn is_multiple(larger: f32, smaller: f32) -> bool {
    // da es zu Rundungsfehlern kommen kann, definieren wir eine erlaubte Abweichung
    let epsilon = 0.001;
    if smaller == 0.0 {
        false
    } else if larger < smaller {
        false
    } else if -epsilon < larger-smaller && larger-smaller < epsilon  {
        true
    } else {
        is_multiple(larger - smaller, smaller)
    }
}

// Aufgabe 2: Typen
fn print_type() {
    // Einen Trait schreiben, so das jeder Datentyp wiedergeben kann was er ist. 
}

