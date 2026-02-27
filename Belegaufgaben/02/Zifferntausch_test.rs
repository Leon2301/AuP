
fn main () {
    let n = 12345;
    let i = 1;
    let j = 3;
    let test = swap_digits(n, i, j);
    println!("Ergebnis: {}", test); // erwarteter Output: 14325
    println!("Ergebnis: {}", swap_digits(1, 0, 0));
    println!("Ergebnis: {}", swap_digits(42, 1, 0));
    println!("Ergebnis: {}", swap_digits(112345, 4, 4));
    println!("Ergebnis: {}", swap_digits(321, 2, 1));
    println!("Ergebnis: {}", swap_digits(187, 0, 2));
}


fn swap_digits(n: u32, i: u32, j: u32) -> u32 {
    //println!("i: {} = {}, j: {} = {}", i,find_digit(n, j), j,find_digit(n, i));
    change_digit(change_digit(n, i, find_digit(n, j)), j, find_digit(n, i))
}

// Gibt den Wert an der Stelle pos Zurück.
fn find_digit(n: u32, pos: u32) -> u32 {
    if pos == 0 {
        n % 10 
    } else {
        find_digit(n/10, pos-1)
    }
}

// Tauscht den Wert an vorgegebener Stelle pos mit new_digit
fn change_digit(n: u32, pos: u32, new_digit: u32) -> u32 {
    if pos == 0 {
        n / 10 * 10 + new_digit
    } else {
        10 * change_digit(n/10, pos-1, new_digit) + n%10
    }
}