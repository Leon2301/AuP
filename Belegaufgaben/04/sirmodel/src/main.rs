const DE: u8 = 3; // Dauer der Anfülligkeit (bis Infektion)
const DI: u8 = 4; // Dauer der Infektion
const DR: u8 = 2; // Dauer der Immunität (bis wieder anfülltig)
type SCHEME = [[State; SCHEME_SIZE]; SCHEME_SIZE];

#[derive(Copy, Clone, PartialEq, Debug)] // Später das Debug wieder raus nehmen!! // LÖSCHEN !!!!!!
// Beschreibt den gesundheitlichen Zustand einer Person
enum State {
    S(u8),  // Anfällig (Dauer der Anfälligkeit = 0-DE) -- Bei 0 hat es keine infizierte Person in der Umgebung. 
    I(u8),  // Infiziert (Dauer der Infizität = 1-DI)
    R(u8),  // Genesung (Dauer der Genesung = 1-DR)
    P       // Permanente Immunittä
}

// Zur Auswahl einer Person
#[derive(Debug)] // LÖSCHEN !!!!!!
struct Coord {
    row: usize, // Zeile
    col: usize  // Spalte
}

// Die Person selbst
#[derive(Debug)] // LÖSCHEN !!!!!!
struct Person {
    coord: Coord, // Das struct Coord
    state: State  // Das enum State
}

// Der eigentliche Simulationsmodel. 
trait SIRModel {
    // erzeugt ein neues SCHEME
    fn initialize(persons: &[Option<Person>; 10]) -> SCHEME;
    // Zählt die Anzahl der Personen die in einem bestimmten Status sind
    fn count_state(&self, target: State) -> u32;
    // führt die Simulation um n Tage weiter 
    fn step_days(&mut self, n: u8);
}

// --- Hier kommt der eigentliche Code ---

// Nun implementieren wir den Trait ersteinmal 
impl SIRModel for SCHEME {
    // erzeugt ein neues SCHEME
    fn initialize(persons: &[Option<Person>; 10]) -> SCHEME {
        // zunächst erstellen wir eine Matrix mit nur Anfälligen Personen. 
        let mut scheme = emty_scheme(); 

        // Jetzt fügen wir hier die einzelnen Personen ein. 
        new_persons(persons, scheme)
    }
    // Zählt die Anzahl der Personen die in einem bestimmten Status sind
    fn count_state(&self, target: State) -> u32 {
        // Gibt den Anzahl der Leute mit dem gleichen Status wieder, dabei ist z.B. S(1) != S(2)
        count_state(self, target)
    }
    // führt die Simulation um n Tage weiter 
    fn step_days(&mut self, n: u8) {
        step_days(self, n);
    }
}

// --- Hilfsfunktionen ---

// leeres SCHEME erstellen: 
fn emty_scheme() -> SCHEME {
    let mut scheme = [[State::S(0); SCHEME_SIZE]; SCHEME_SIZE];
    scheme
}

// setzt die Personen an der jeweiligen Stelle ein.
fn new_persons(persons: &[Option<Person>; 10], mut scheme: SCHEME) -> SCHEME {
    // fügt rekursiv die neuen Personen ein. 
    fn new_persons_helper(persons: &[Option<Person>; 10], mut scheme: SCHEME, idx: usize) -> SCHEME {
        // Für die Abbruchbedingung verwende ich direkt 9, da wir .len() laut Aufgabenstellung nicht verwenden dürfen. 
        
        if idx == 9 {
            // Hier prüfen wir ob es überhaupt ein Some() in dem Option Typ gibt: 
                // Da die Werte von Persons nur geliehen sind und Person nicht copy ist, könnnen wir auch deren "childs" nicht direkt nutzen. 
                // Der Trick ist nun das wir mit .as_ref() eine Referenz davon anlegen und diese entpacken. 
                // let person = persons[idx].as_ref().unwrap();
                // Anscheinend darf man das nicht, daher hier die elegante Variante: 
            if let Some(person) = &persons[idx] {
                // Da Person nicht clone ist müssen wir hier eine Referenz auf die Elemente übergeben. 
                let coord: &Coord = &person.coord;
                let state: &State = &person.state;
                scheme[coord.row][coord.col] = *state; 
            }
            scheme
        } else {
            if let Some(person) = &persons[idx] {
                let coord: &Coord = &person.coord;
                let state: &State = &person.state;
                scheme[coord.row][coord.col] = *state; 
            }
            // Der rekursive Aufruf soll immer geschehen, egal ob eine Person übergeben wurde oder nicht. 
            new_persons_helper(persons, scheme, idx +1)
        }
    }
    // Hier wird der Helper direkt aufgerufen mit Index, hat den Vorteil das man beim Aufruf von new_persons keinen Index übergeben muss. 
    new_persons_helper(persons, scheme, 0)
}

// Gibt zurück wie viele States in einem bestimmten Zustand in der Matrix vorhanden sind. 
fn count_state(scheme: &SCHEME, target: State) -> u32 {
    // Soll durch alle Spalten und Zeilen rekursiv durchgehen
    fn count_state_helper(
        scheme: &SCHEME, 
        target: State, 
        row: usize,
        col: usize, 
        scheme_size: usize // Da die Matrix Quadratisch ist, können wir das für Zeilen und Spalten verwenden. 
    ) -> u32 {
        // Hier gehen wir nacheinander die Spalte(col) durch und dann zur nächsten Zeile(row). 
        // Da scheme_size immer eins länger als die Indizes, müssen wir in der Abbruchbedingung nichts mehr machen. 
        if row == scheme_size {
            0
        } else {
            if col == scheme_size {
                // Springt in die nächste Zeile
                count_state_helper(scheme, target, row+1, 0, scheme_size)
            } else {
                // Hier vergleichen wir den eigentlichen Status. 
                // Bisher ist mir noch keine bessere Variante eingefallen als jeden Status einzeln abzufangen....
                if let State::S(_) = scheme[row][col] {
                    if let State::S(_) = target {
                        1 + count_state_helper(scheme, target, row, col+1, scheme_size)
                    } else {
                        count_state_helper(scheme, target, row, col+1, scheme_size)
                    } 
                } else if let State::I(_) = scheme[row][col] {
                    if let State::I(_) = target {
                        1 + count_state_helper(scheme, target, row, col+1, scheme_size)
                    } else {
                        count_state_helper(scheme, target, row, col+1, scheme_size)
                    } 
                } else if let State::R(_) = scheme[row][col] {
                    if let State::R(_) = target {
                        1 + count_state_helper(scheme, target, row, col+1, scheme_size)
                    } else {
                        count_state_helper(scheme, target, row, col+1, scheme_size)
                    }
                } else {
                    if let State::P = target {
                        1 + count_state_helper(scheme, target, row, col+1, scheme_size)
                    } else {
                        count_state_helper(scheme, target, row, col+1, scheme_size)
                    }
                }
            }
        }
    }
    count_state_helper(scheme, target, 0, 0, SCHEME_SIZE)
}

// Rekursiver durchlauf durch n Tage.  
fn step_days(scheme: &mut SCHEME, n: u8) {
    fn step_days_helper(scheme: &mut SCHEME, n: u8, idx: usize) {
        if idx == n as usize {
            step_one_day(scheme);
        } else {
            step_one_day(scheme);
            step_days_helper(scheme, n, idx+1);
        }
    }
    step_days_helper(scheme, n, 1);
}

fn step_one_day(scheme: &mut SCHEME) {
    // 1. Schaut ob sich der Status des Nachbarn auf meinen auswirkt. 

    // Zur Hilfe von S(0) -> S(1) nehmen wir zur Hilfe eine Matrix
    type MATRIX = [[u8; SCHEME_SIZE]; SCHEME_SIZE];
    let mut help_s: MATRIX = [[0; SCHEME_SIZE]; SCHEME_SIZE];

    // Dafür bruachen wir erstmal wieder eine Funktion die durch alle Elemente durch geht: 
    fn compare_with_helper(scheme: &mut SCHEME, row: usize, col: usize, scheme_size: usize, help_s: &mut MATRIX) -> MATRIX {
        // Hier gehen wir nacheinander die Spalte(col) durch und dann zur nächsten Zeile(row). 
        // Da scheme_size immer eins länger als die Indizes, müssen wir in der Abbruchbedingung nichts mehr machen. 
        if row == scheme_size {
            *help_s
        } else {
            if col == scheme_size {
                // Springt in die nächste Zeile
                compare_with_helper(scheme, row+1, 0, scheme_size, help_s)
            } else {
                // gheen hier davon aus das wir in alle Richtungen Nachbarn haben. 

                // Schaut ob sich der Status des Nachbarn auf meinen auswirkt. 

                // schauen ob Nachbarn infiziert sind. 
                // Außerdem muss jedes mal überprüft werden ob der Zugriff überhaupt erlaubt ist. 
                let mut i_neighbors = 0; 
                if row < scheme_size-1 {  if let State::I(_) = scheme[row+1][col] { i_neighbors += 1; } }
                if col < scheme_size-1 {  if let State::I(_) = scheme[row][col+1] { i_neighbors += 1; } }
                if row > 0 {            if let State::I(_) = scheme[row-1][col] { i_neighbors += 1; } }
                if col > 0 {            if let State::I(_) = scheme[row][col-1] { i_neighbors += 1; } }
                
                // bei zwei infezierten Nachbarn wird sofort infiziert 
                // Da wir später noch hochzählen, müssen wir das hier bedenken. 
                if i_neighbors >= 2 {
                    // Regel 4
                    if let State::S(_) = scheme[row][col] {
                        //scheme[row][col] = State::I(0);
                        help_s[row][col] = 2;
                    }
                    compare_with_helper(scheme, row, col+1, scheme_size, help_s)
                } else if i_neighbors == 1 {
                    // hat einen infizierten Nachbarn
                    // Wenn S(0) dann auf S(1), diese Informationen speicher wir in help_s
                    // Teil von Regel 2
                    if let State::S(0) = scheme[row][col] {
                        help_s[row][col] = 1;
                    }
                    compare_with_helper(scheme, row, col+1, scheme_size, help_s)
                } else {
                    // Regel 1 und Regel 3
                    if let State::S(_) = scheme[row][col] {
                        scheme[row][col] = State::S(0);
                    }
                    compare_with_helper(scheme, row, col+1, scheme_size, help_s)
                }
            }
        }
    }
    help_s = compare_with_helper(scheme, 0, 0, SCHEME_SIZE, &mut help_s);


    // 2. Schauen ob sich der eigene Status ändert. (z.B. S(4) -> i(1) )
    fn status_change_helper(scheme: &mut SCHEME, row: usize, col: usize, scheme_size: usize ) {
        if row == scheme_size {
            // ende...
        } else {
            if col == scheme_size {
                // Springt in die nächste Zeile
                status_change_helper(scheme, row+1, 0, scheme_size);
            } else {
                // Wir schauen ob der Status wechselt/
                if let State::S(DE) = scheme[row][col]{ 
                    // teil von Regel 2
                    scheme[row][col] = State::I(0);
                } else if let State::I(DI) = scheme[row][col]{
                    // Regel 5
                    scheme[row][col] = State::R(0);
                } else if let State::R(DR) = scheme[row][col]{
                    // Regel 6
                    scheme[row][col] = State::S(0);
                }
                status_change_helper(scheme, row, col+1, scheme_size);
            }
        }
    }
    status_change_helper(scheme, 0, 0, SCHEME_SIZE);

    // Zahlen erhöhen
    fn count_up_helper(scheme: &mut SCHEME, row: usize, col: usize, scheme_size: usize, help_s: &MATRIX) {
        if row == scheme_size {
            // ende...
        } else {
            if col == scheme_size {
                // Springt in die nächste Zeile
                count_up_helper(scheme, row+1, 0, scheme_size, help_s);
            } else {
                // Hier wird noch der Fall S(0) abgefangen und danach alle um eins erhöht. 
                if help_s[row][col] == 2 {
                    // Teil von Regel 4
                    scheme[row][col] = State::I(1);
                } else if State::S(0) != scheme[row][col] {
                    if let State::S(value) = scheme[row][col] {
                        scheme[row][col] = State::S(value+1);
                    } else if let State::I(value) = scheme[row][col] {
                        scheme[row][col] = State::I(value+1);
                    } else if let State::R(value) = scheme[row][col] {
                        scheme[row][col] = State::R(value+1);
                    } // Somit Regel 7 erfüllt
                } else if help_s[row][col] == 1 {
                    // Wird nur erreicht wenn S(0) und help_s an der Stelle true.
                    // teil von Regel 2
                    scheme[row][col] = State::S(1);
                }
                count_up_helper(scheme, row, col+1, scheme_size, help_s);
            }
        }
    }
    count_up_helper(scheme, 0, 0, SCHEME_SIZE, &help_s);
}


// nur für Test zwecke, muss später noch entfernt werden. 
// Wird vom Aufgabensteller gegeben. 
const SCHEME_SIZE: usize = 8; 
fn main() {
    println!("Hello, world!");

    // Das könnte ein Beispiel sein für Die 10 Personen. Der Rest wird immer mit S(0) aufgefüllt. 
    let initial_persons: [Option<Person>; 10] = [
    // 3 Ausbruchsorte
    Some(Person { coord: Coord{row:1, col:1}, state: State::I(1)}),
    //Some(Person { coord: Coord{row:7, col:7}, state: State::I(4)}),
    Some(Person { coord: Coord{row:1, col:6}, state: State::I(1)}),
    Some(Person { coord: Coord{row:6, col:3}, state: State::I(1)}),
    // Personen mit permanenter Immunität
    Some(Person { coord: Coord{row:3, col:3}, state: State::P}),
    Some(Person { coord: Coord{row:4, col:3}, state: State::P}),
    Some(Person { coord: Coord{row:3, col:4}, state: State::P}),
    Some(Person { coord: Coord{row:4, col:4}, state: State::P}),
    // nicht benutzte Positionen im Array
    None,
    None,
    None
    ];

    // --- Ein paar Tests ---
    let mut test_set: SCHEME = SCHEME::initialize(&initial_persons); 
    let test_p: u32 = test_set.count_state(State::P);
    let test_i: u32 = test_set.count_state(State::I(1));

    // Tag: 0
    println!("--- TAG: 0 ---");

    for i in test_set {
        println!("{:?}", i);
    }
    println!("Personen P: {}", test_p);
    println!("Personen I: {}", test_i);

    // Zählt n Tage nach vorne
    test_set.step_days(10);

    for i in test_set {
        println!("{:?}", i);
    }
    println!("Personen P: {}", test_set.count_state(State::P));
}

/*
Test ausgaben: 

print!("{:?} ", scheme[row][col]);
println!("({}, {}) hat zwei infizierte Nachbarn. ", row, col);

 // Hier ein paar Test später // Löschen !!!!!!
                //println!("Person {:?}", person);
                println!("Person: {}, coord: {:?}, state: {:?}",idx+1, coord, state);
                //println!("Person: {}, coord: {:?}, state: {:?}",idx+1, coord, state);
                println!("Coord: {:?},{:?}", coord.row, coord.col);
                println!("State: {:?}", state);
                println!("{:?}", scheme[coord.row][coord.col]);

*/