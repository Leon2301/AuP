trait Necklace {
    fn get_len(self) -> usize; 
    fn is_valid(self) -> bool;
    fn is_identical_with(self, other: Self) -> bool; 
}

impl Necklace for [(u8,u8,u8);20] {
    // gibt die Anzahl der Elemente die nicht (0,0,0) sind.
    fn get_len(self) -> usize {
        count_chain_elements(self, 0)
    }

    // gibt false, falls es mehrere Lücken gibt, sonst true
    fn is_valid(self) -> bool {
        // wenn es mehr als eine große Lücke gibt, wird ein false zurückgegeben. 
        if count_emty_spaces(self, 0, false) > 1 {
            false
        } else {
            true
        }
    }

    // gibt true, wenn es eine ähnliche Varianten sind, sonst false
    fn is_identical_with(self, other: Self) -> bool {
        // Überprüft ob keine der Ketten eine Nullreihe ist. 
        if 0 < self.get_len() && 0 < other.get_len() {
                // Vergleicht auf: Identisch, rotiert, gespiegelt, rotiert & gespiegelt. 
            if compare_with(self, other, 0) {
                true
            // Die Perlen können nur noch in einer anderen Reihenfolge dargestellt worden sein, wenn es auch leere Felder gibt. 
            // Da bereits vorgeschrieben wurde, dass es nur eine Lücke geben darf, ist dies hier ausreichend.
            // Außerdem behandeln wir nur Arrays der länge 20, weshalb im folgenden auch immer gleich die 20 geschrieben wird. 
            } else if self.get_len() < 20 {
                // Nun möchten wir noch schauen ob die Perlen vertauscht wurden. 
                switched_order(self, other, 0)
            } else {
                false
            }
        } else if 0 == self.get_len() && 0 == other.get_len() {
            true
        } else {
            false
        }
    }
}

// --- Hilfsfunktionen ---

// zählt die zusammenhängenden Lücken
// chain ist das Perlenketten Array, idx ist der Index im Array, space_before gibt an ob davor schon eine Lücke war. 
fn count_emty_spaces(chain: [(u8,u8,u8);20], idx: usize, space_before: bool) -> i8 {
    // Abbruchbedingung. 
    if idx == 19 {
        // schaut ob das letzte Element leer ist.
        if chain[idx] == (0,0,0) {
            // schaut ob auch das erste Elemnt leer ist.
            if chain[0] == (0,0,0) {
                // schaut ob das Element davor schon leer war, falls ja wurde eine Lücke zu viel gezählt. 
                if space_before {
                    -1
                } else {
                    0
                }
            } else {
                // schaut ob das Element davor schon leer war. 
                if space_before {
                    0
                } else {
                    1
                }
            }
        } else { 
            // Perle an dieser Stelle
            0
        }
    } else {
        // Beginn des Rekursionsschrittes
        if chain[idx] == (0,0,0) {
            // schaut ob das Element davor schon leer war.
            if space_before {
                // wir befinden uns noch in der selben Lücke.
                0 + count_emty_spaces(chain, idx+1, true)
            } else {
                1 + count_emty_spaces(chain, idx+1, true)
            }
        } else {
            // Perle an dieser Stelle
            0 + count_emty_spaces(chain, idx+1, false)
        }
    }
}

// gibt die Anzahl der Elemente die nicht (0,0,0) sind.
fn count_chain_elements(chain: [(u8,u8,u8);20], idx: usize) -> usize{
    // Abbruchbedingung: am Ende von chain
    if idx == 19 {
        // Wenn das Element nicht leer ist addiere 1, ansonstn null
        if chain[idx] == (0,0,0) {
            0
        } else {
            1
        }
    // Rekursionsschritt
    } else {
        // Wenn das Element nicht leer ist addiere 1, ansonstn null
        if chain[idx] == (0,0,0) {
            0 + count_chain_elements(chain, idx+1)
        } else {
            1 + count_chain_elements(chain, idx+1)
        }
    }
}

// Vergleicht chain mit other. Wobei other rotiert ist um rotated_by. 
fn compare_with(chain: [(u8,u8,u8);20], other: [(u8,u8,u8);20], rotated_by: usize) -> bool {
    // Abbruchbedingung, bei der die aufgerufene Funktion direkt das Ergebnis weitergibt.
    if rotated_by == 19 {
        compare_with_helper(chain, other, rotated_by, 0)
    } else {
        // Wenn hier ein übereinstimmung gefunden wurde, wird direkt true zurückgegeben, ansonsten probieren wir es weiter. 
        if compare_with_helper(chain, other, rotated_by, 0) == true {
            true
        } else {
            // rekursiver Aufruf, wobei die Rotation um eine Stelle weiter geschoben wird.
            compare_with(chain, other, rotated_by+1)
        }
    }
}

// Wir gehen hier wieder rekursiv durch und vergleichen die Einträge nacheinander. 
fn compare_with_helper(chain: [(u8,u8,u8);20], other: [(u8,u8,u8);20], rotated_by: usize, idx: usize) -> bool {
    // Abbruchbedingung: wenn der Index durch alle Elemente einmal durch ist. 
    if idx == 19{
        // Hier vergleichen wir mit other. Wenn wir über die Länge des Arrays hinausgehen, 
        // springen wir mit % wieder nach vorne: z.B idx = 19  --> [19] == [(19+1) % 20 = 0]
        if chain[idx] == other[(idx + rotated_by)%20] {
            true
        // hier testet er außerdem noch die Spiegelung.
        // Dafür Ziehen wir einfach den aktuellen Index vom Masimalen ab.  
        } else if chain[idx] == other[19 - (idx + rotated_by)%(20)]{
            true
        }else {
            false
        }
    } else {
        // Vergleicht wieder rotierte Variante.
        if chain[idx] == other[(idx + rotated_by)%(20)] {
            compare_with_helper(chain, other, rotated_by, idx+1)
        // Vergleicht wieder rotiert gespiegelte Variante. 
        } else if chain[idx] == other[19 - (idx + rotated_by)%(20)]{
            compare_with_helper(chain, other, rotated_by, idx+1)
        // Wenn bereits ein Element nicht übereinstimmt ist der gesmate Vergleich false. 
        }else {
            false
        }
    }
}

fn switched_order(chain: [(u8,u8,u8);20], other: [(u8,u8,u8);20], rotated_by: usize) -> bool {
    // Prüft ob die Kette überhaupt gleich viele Eelemente hat
    if count_chain_elements(chain, 0) == count_chain_elements(other, 0) {
        // rotiert die Reihenfolge der inneren Perlen und ruft zum Vergleichen auf. 
        if rotated_by == count_chain_elements(chain, 0) -1 {
            switched_order_helper(chain, other, starting_point(chain), starting_point(other), rotated_by, count_chain_elements(chain,0), 0)
        } else {
            // Wenn hier ein übereinstimmung gefunden wurde, wird direkt true zurückgegeben, ansonsten probieren wir es weiter. 
            if  switched_order_helper(chain, other, starting_point(chain), starting_point(other), rotated_by, count_chain_elements(chain,0), 0) == true {
                true
            } else {
                // rekursiver Aufruf, wobei die Rotation um eine Stelle weiter geschoben wird.
                switched_order(chain, other, rotated_by+1)
            }
        }
    } else {
        false
    }
}

// funktioniert wie der compare_with_helper nur mit einer bestimmten Länge der Perlenkette
// Er geht rekursiv durch alle gefüllten Elemente und vergleicht diese in Rotation.
fn switched_order_helper(
    chain: [(u8,u8,u8);20], 
    other: [(u8,u8,u8);20], 
    chain_s: usize, // Index an der die erste Perle kommt
    other_s: usize, // Index an der die erste Perle kommt
    rotated_by: usize, 
    elemente: usize, 
    idx: usize
) -> bool {
    // Abbruchbedingung: wenn der Index durch alle Elemente einmal durch ist. 
    if idx == elemente-1 {
        // Hier vergleichen wir mit other. Wenn wir über die Länge des Arrays hinausgehen, 
        // springen wir mit modulo wieder nach vorne
        if chain[(idx+chain_s)%(20)] == other[((idx + rotated_by)%(elemente) + other_s)%(20)] {
            true
        // hier testet er außerdem noch die Spiegelung.
        // Dafür Ziehen wir einfach den aktuellen Index vom höchsten ab.  
        } else if chain[(idx+chain_s)%(20)] == other[(elemente -1 - (idx + rotated_by)%(elemente) + other_s)%(20)]{
            true
        }else {
            false
        }
    } else {
        // Vergleicht wieder rotierte Variante.
        if chain[(idx+chain_s)%(20)] == other[((idx + rotated_by)%(elemente) + other_s)%(20)] {
            switched_order_helper(chain, other, chain_s, other_s, rotated_by, elemente, idx+1)
        // Vergleicht wieder rotiert gespiegelte Variante. 
        } else if chain[(idx+chain_s)%(20)] == other[(elemente -1 - (idx + rotated_by)%(elemente) + other_s)%(20)]{
            switched_order_helper(chain, other, chain_s, other_s, rotated_by, elemente, idx+1)
        // Wenn bereits ein Element nicht übereinstimmt ist der gesmate Vergleich false. 
        }else {
            false
        }
    }
}

// gibt den Index der erste Perle zurück oder 0
fn starting_point(chain: [(u8,u8,u8);20]) -> usize {
    if count_emty_spaces(chain, 0, false) == 1 {
        starting_point_helper(chain, false, 0)
    } else {
        0
    }
}

fn starting_point_helper(chain: [(u8,u8,u8);20], space_before: bool, idx: usize) -> usize {
    // Wir wissen bereits aus der Mutterfunktion, das es eine zusammenhängende Lücke von (0,0,0) Elementen gibt. 
    if space_before == true {
        if chain[idx] != (0,0,0) {
            idx
        } else {
            starting_point_helper(chain, true, (idx+1)%(20))
        }
    } else {
        if chain[idx] == (0,0,0) {
            starting_point_helper(chain, true, (idx+1)%(20))
        } else {
            starting_point_helper(chain, false, (idx+1)%(20))
        }
    }
}
