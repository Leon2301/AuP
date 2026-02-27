fn swap_digits(n: u32, i: u32, j: u32) -> u32 {
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