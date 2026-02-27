// Consts for cars (kilometer)
const MAINTENANCE_KM: u32 = 5000;
const TUV_KM: u32 = 15000;
// Consts for cars (days)
const MAINTENANCE_DAYS: u32 = 2;
const TUV_DAYS: u32 = 3;
// Consts for retirement 
const MAX_AGE_DAYS: u32 = 3650;
const MAX_KM: u32 = 200000;
const MAX_RENTALS: u32 = 500;

#[derive(Clone, PartialEq, Eq)]
enum PersonStatus {
    Active,
    Blocked,
}

#[derive(Clone, PartialEq, Eq)]
enum CarStatus {
    Available,
    Rented,
    Maintenance(u32),
    Tuv(u32),
    Retired,
}

struct Person<'a> {
    identifier: &'a str,
    license_valid_days: u32,
    status: PersonStatus,
}

struct Car<'a> {
    identifier: &'a str,
    mileage: u32,
    status: CarStatus,
    age_days: u32,
    rental_count: u32,
}

struct Reservation<'a> {
    person_id: &'a str,
    car_id: &'a str,
    priority: u32,
}

struct CarSharing<'a> {
    persons: Vec<Person<'a>>,
    cars: Vec<Car<'a>>,
    rentals: Vec<(&'a str, &'a str)>,
    reservations: Vec<Reservation<'a>>,
    current_day: u32,
}

trait CarSharingService<'a> {
    fn register_person(&mut self, p: Person<'a>) -> bool;
    fn unregister_person(&mut self, identifier: &str) -> bool;
    fn renew_license(&mut self, identifier: &str, new_valid_days: u32) -> bool;
    fn get_person_status(&self, identifier: &str) -> Option<PersonStatus>;

    fn register_car(&mut self, c: Car<'a>) -> bool;
    fn unregister_car(&mut self, identifier: &str) -> bool;
    fn get_car_status(&self, identifier: &str) -> Option<CarStatus>;


    fn get_available_cars(&self) -> Vec<&str>; 
    fn reserve_car(&mut self, person_id: &'a str, car_id: &'a str, priority: u32) -> bool;
    fn cancel_reservation(&mut self, person_id: &str, car_id: &str) -> bool;  
    fn get_reservations_for_car(&self, car_id: &str) -> Vec<&str>; 
    fn process_reservations(&mut self) -> Vec<(&'a str, &'a str)>; 

    fn rent_car(&mut self, person_id: &'a str, car_id: &'a str) -> bool;
    fn return_car(&mut self, person_id: &str, car_id: &str, driven_km: u32) -> bool;

    fn simulate_n_days(&mut self, n: u32);   
}

impl<'a> CarSharingService<'a> for CarSharing<'a> {
    fn register_person(&mut self, p: Person<'a>) -> bool {
        // Um dem Carsharing beizutreten, muss die ID eindeutig sein. 
        let iterator_person_id = (&self.persons).into_iter().map(|x| x.identifier); // Iterator... (siehe register_car)
        if is_valid(iterator_person_id, &p.identifier) {
            // Carsharing beitreten 
            self.persons.push(p);
            true
        } else {
            false
        }
    } // done 
    fn unregister_person(&mut self, identifier: &str) -> bool {
        // Keine Fahrzeuge ausgeliehen ??
        if has_no_car(&self.rentals, identifier) {
            // Lösche alle Reservierungen
            // iterriert durch alle Fahrzeuge und löscht gegebenenfalls die Reservierung zu Car. 
            for idx in 0..self.cars.len() {
                self.cancel_reservation(identifier, self.cars[idx].identifier);
            }
            // Lösche Person
            if let Some(per_idx) = get_person_index(&self.persons, identifier) { // sollte immer true sein. 
                self.persons.remove(per_idx);
            }
            return true
        }
        false
    } // done
    fn renew_license(&mut self, identifier: &str, new_valid_days: u32) -> bool {
        // prüft ob es die Person gibt und macht dann die Zuweisung.
        if let Some(idx) = get_person_index(&self.persons, &identifier) {
            self.persons[idx].license_valid_days = new_valid_days; 
            true
        } else {
            false
        }
    } // done
    fn get_person_status(&self, identifier: &str) -> Option<PersonStatus> {
        // prüft ob es die Person gibt und macht dann die Zuweisung.
        if let Some(idx) = get_person_index(&self.persons, &identifier) {
            let status = self.persons[idx].status.clone(); 
            Some(status)
        } else {
            None
        }
    } // done

    fn register_car(&mut self, c: Car<'a>) -> bool {
        // Um dem Carsharing beizutreten, muss die ID eindeutig sein. 
        // iterator_car_ID verwandelt die Referenzen auf cars in einen Iterator (dadurch werden die Cars nicht verbraucht.), dann wird mit dem .map nicht ein ganzes Car sondern immer nur die ID jedes Cars als Item übergeben. Daduch konnte ich die is_valid Funktion allgemein gültig machen und nicht nur für Person oder Car. Wie Geil!
        let iterator_car_id = (&self.cars).into_iter().map(|x| x.identifier);
        if is_valid(iterator_car_id, &c.identifier) {
            // Prüfe ob es bereits zu alt ist.
            // Wusst aus der Aufgabenstellung jetzt nicht genau ob das notwenidg ist. 
            if !must_car_retire(c.age_days, c.mileage, c.rental_count) {
                // Carsharing beitreten 
                self.cars.push(c);
                return true
            }
        }
        false
    } // done
    fn unregister_car(&mut self, identifier: &str) -> bool {
        // geht nur wenn es nicht ausgeliehen, in der Wartung oder beim TÜV ist. Also Available oder Retired
        let mut del = false; 
        if let Some(CarStatus::Available) = self.get_car_status(identifier) { del = true }
        if let Some(CarStatus::Retired) = self.get_car_status(identifier) { del = true }
        if del {
            // Lösche alle Reservierungen
            // iterriert durch alle Personen und löscht gegebenenfalls die Reservierung zu Car. 
            for idx in 0..self.persons.len() {
                self.cancel_reservation(self.persons[idx].identifier, identifier);
            }
            
            if let Some(car_idx) = get_car_index(&self.cars, identifier) { // sollte immer true sein. 
                // Setze Fahrzeug auf Retired. unnötig.
                // Retired Fahrzeuge können gelöscht werden
                self.cars.remove(car_idx);
            }
            return true
        }
        false
    } // done
    fn get_car_status(&self, identifier: &str) -> Option<CarStatus> {
        // prüft ob es das Auto gibt und macht dann die Zuweisung.
        if let Some(idx) = get_car_index(&self.cars, &identifier) {
            let status = self.cars[idx].status.clone(); 
            Some(status)
        } else {
            None
        }
    } // done


    fn get_available_cars(&self) -> Vec<&str> {
        // geht iterativ durch alle Fahrzeuge und schreibt die "Available" Cars in einen Vektor.
        let mut ava_cars = Vec::<&str>::new();
        for i in &self.cars {
            if i.status == CarStatus::Available {
                ava_cars.push(i.identifier);
            }
        }
        // Man hätte das auch mit self.get_car_status machen können, doch das ging einfacher, da ich die identifier nicht kennen muss. 
        // Alternativ hätten wir auch mit Adaptern arbeiten können.
        ava_cars
    } // done
    fn reserve_car(&mut self, person_id: &'a str, car_id: &'a str, priority: u32) -> bool {
        if priority < 101 {
            // bereits ein Fahrzeug ausgeliehen? 
            if has_no_car(&self.rentals, &person_id) {
                // bereits eine Rservierung auf dieses Fahrzeug von der gleichen Person? 
                if !reservated(&self.reservations, &person_id, &car_id) {
                    // is die Person active. 
                    if let Some(PersonStatus::Active) = self.get_person_status(&person_id) {
                        let new_reserve: Reservation = Reservation {
                            person_id: person_id,
                            car_id: car_id,
                            priority: priority,
                        };
                        self.reservations.push(new_reserve);
                        return true
                    }
                }
            }
        }
        false
    } // done
    fn cancel_reservation(&mut self, person_id: &str, car_id: &str) -> bool {
        // Prüft ob es diese Reservierung überhaupt gibt. 
        if reservated(&self.reservations, &person_id, &car_id) {
            // Es gibt eine Reservierung
            for (idx, r) in (&self.reservations).into_iter().enumerate() {
                let p = r.person_id;
                let c = r.car_id;
                // Wenn wir und an diesem Eintrag befinden, lösche diesen.  
                if p == person_id && c == car_id {
                    self.reservations.remove(idx); 
                    // Da es immer nur eine Reservierung zwischen der gleichen Person und Car geben kann, können wir hier direkt returnen. 
                    return true
                }
            }; 
        }
        // es besteht keine Reservierung 
        false
    } // done 
    fn get_reservations_for_car(&self, car_id: &str) -> Vec<&str> {
        // iteriert durch alle Reservierungen durch und speichert die Personen in den Vektor welche eine Reservierung auf dieses Fahrzeug gemacht haben. 
        let mut result = Vec::<&str>::new(); 
        for r in &self.reservations {
            if r.car_id == car_id {
                // Es besteht bereits eine Reservierung der Art. 
                result.push(r.person_id);
            }
        }
        result
    } // done
    fn process_reservations(&mut self) -> Vec<(&'a str, &'a str)> {
        let mut result = Vec::<(&str, &str)>::new();

        // sortiere alle Reservierungen nach Priorität. groß nach klein
        bsort_priority(&mut self.reservations); 

        // iterriere durch die Liste und prüfe Bedingungen: 
        for reserve in &self.reservations {
            // Person hat noch kein Fahrzeug
            if has_no_car(&self.rentals, reserve.person_id) {
                // Person hat noch kein Fahrzeug zugewiesen bekommen. 
                if has_no_car(&result, reserve.person_id) {
                    // Fahrzeug verfügbar?
                    let ava_cars = self.get_available_cars(); // erzeugt eine Liste an verfügbaren Fahrzeugen
                    if ava_cars.iter().any(|x| x == &reserve.car_id){// schaut ob das gewünschte Fahrzeug in der Liste ist. 
                        // Fahrzeug hat noch kein Person zugewiesen bekommen. 
                        if has_no_person(&result, reserve.car_id) {
                            // Gültiger Führerschein (Available)?
                            if let Some(p) = get_person_index(&self.persons, reserve.person_id) {
                                if self.persons[p].status == PersonStatus::Active {
                                    //self.rent_car(&reserve.person_id, &reserve.car_id); // Wie kann ich diese Werte verleihen, ohne das der Borrowchecker sich beschwert?
                                    result.push((reserve.person_id, reserve.car_id));
                                }
                            }
                        }
                    }
                }
            }
        }
        // Verwandle alle Paare in Ausleihen. 
        for (person_id, car_id) in &result {
            self.rent_car(person_id, car_id);
        }
        result
    } // done

    fn rent_car(&mut self, person_id: &'a str, car_id: &'a str) -> bool { 
        // selbe überprüfungen wie bei process_reservations (gut das Effiziens hier egal ist...)
        if has_no_car(&self.rentals, person_id) {
            // Fahrzeug verfügbar?
            let ava_cars = self.get_available_cars(); // erzeugt eine Liste an verfügbaren Fahrzeugen
            if ava_cars.iter().any(|x| x == &car_id){ // schaut ob das gewünschte Fahrzeug in der Liste ist. 
                // Gültiger Führerschein (Available)?
                if let Some(p) = get_person_index(&self.persons, person_id) {
                    if self.persons[p].status == PersonStatus::Active {
                        // Fahrzeug darf verliehen werden. 

                        // lösche alle Reservierungen von dieser Person. 
                        // iterriert durch alle Fahrzeuge und löscht gegebenenfalls die Reservierung zu Car. 
                        for idx in 0..self.cars.len() {
                            self.cancel_reservation(person_id, self.cars[idx].identifier);
                        }

                        // schreibe die rentels um
                        self.rentals.push((person_id, car_id));
                        // Setze den Status des Autos auf vermietet (Rented).
                        if let Some(car_idx) = get_car_index(&self.cars, car_id) { // sollte immer true sein. 
                            self.cars[car_idx].status = CarStatus::Rented;
                            // Erhöhe die rental_count vom Auto. 
                            self.cars[car_idx].rental_count += 1; 
                        }
                        return true
                    }
                }
            }
        }
        false 
    } // done
    fn return_car(&mut self, person_id: &str, car_id: &str, driven_km: u32) -> bool {
        // mit dem For gehen wir durch alle laufenden Verleihungen um den Index herauszufinden und wenn diese nicht gefunden wird, wird anschließend sowieso false zurück gegeben. 
        for idx in 0..self.rentals.len() {
            if self.rentals[idx].0 == person_id && self.rentals[idx].1 == car_id {
                // Lösche die Verbindung in rentals. 
                self.rentals.remove(idx);
                // nun benötigen wir noch den Index des Autos selbst in der Liste aller Autos. 
                if let Some(car_idx) = get_car_index(&self.cars, car_id) { // sollte immer true sein. 
                    // Ist die Ausmusterung fällig? 
                    let age = self.cars[car_idx].age_days;
                    let km = self.cars[car_idx].mileage + driven_km;
                    let rental_count = self.cars[car_idx].rental_count;
                    if must_car_retire(age, km, rental_count) {
                        self.cars[car_idx].status = CarStatus::Retired;
                        // Da ausgemusterte Fahrzeuge auch entfernt werden dürfen, machen wir das hier gleich ordentlich. 
                        self.unregister_car(car_id);
                    } else {
                        // Setzte den passenden Status
                        // Prüfe ob eine Wartung oder TüV ansteht. (Erst TüV, dann Wartung.)
                        if self.cars[car_idx].mileage % TUV_KM + driven_km >= TUV_KM {
                            // TÜV fällig
                            self.cars[car_idx].status = CarStatus::Tuv(TUV_DAYS);
                        } else if self.cars[car_idx].mileage % MAINTENANCE_KM + driven_km >= MAINTENANCE_KM {
                            // Wartung fällig 
                            self.cars[car_idx].status = CarStatus::Maintenance(MAINTENANCE_DAYS);
                        } else {
                            // Direkt wieder verfügbar.
                            self.cars[car_idx].status = CarStatus::Available;
                        }
                        // Übertrage die neuen Kilometerstand.
                        self.cars[car_idx].mileage += driven_km;
                    }
                }
                return true
            }
        }
        false
    } // done

    fn simulate_n_days(&mut self, n: u32){
        // führe die Tagessimulation n mal durch 
        for _ in 0..n {
            // Zähle die Tage eins hoch. 
            self.current_day += 1; 
            
            // iterriere durch alle Personen
            for idx in 0..self.persons.len() {
                // Zähle die Gültigkeiten der Führerscheine eins runter.
                if 0 < self.persons[idx].license_valid_days {
                    self.persons[idx].license_valid_days -= 1;
                }
                // prüfe die Gültigkeit der Führerscheine
                if 1 > self.persons[idx].license_valid_days {
                    self.persons[idx].status = PersonStatus::Blocked ;
                }
            }
            
            // iterriere duch alle Fahrzeuge mit Hilfe des Index
            for car_idx in 0..self.cars.len() {
                // Erhöhe das Alter von jedem Auto um eins. 
                self.cars[car_idx].age_days += 1; 
                // Zähle die Länge von TÜV und Wartung eins runter
                if let CarStatus::Maintenance(i) = self.cars[car_idx].status {
                    self.cars[car_idx].status = CarStatus::Maintenance(i-1);
                } else if let CarStatus::Tuv(j) = self.cars[car_idx].status {
                    self.cars[car_idx].status = CarStatus::Tuv(j-1);
                }

                // prüfe ob TÜV oder Wartung beendet sind. 
                if let CarStatus::Maintenance(0) = self.cars[car_idx].status {
                    self.cars[car_idx].status = CarStatus::Available;
                } else if let CarStatus::Tuv(0) = self.cars[car_idx].status {
                    self.cars[car_idx].status = CarStatus::Available;
                }

                // prüfe ob bei nicht verliehenen Fahrzeugen eine Ausmusterung ansteht. 
                if CarStatus::Rented == self.cars[car_idx].status {
                    continue
                } else {
                    let age = self.cars[car_idx].age_days;
                    let km = self.cars[car_idx].mileage;
                    let rental_count = self.cars[car_idx].rental_count;
                    if must_car_retire(age, km, rental_count) {
                        self.cars[car_idx].status = CarStatus::Retired;
                        // Da ausgemusterte Fahrzeuge auch entfernt werden dürfen, machen wir das hier gleich ordentlich. 
                        self.unregister_car(self.cars[car_idx].identifier);
                    }
                }
            }

            // Verarbeite die Reservierungen. 
            self.process_reservations();
        }
    } // done
}

// --- Hilfsfunktionen ---

// Prüft ob die ID von identifier einmalig ist. 
fn is_valid<'a>(liste: impl IntoIterator<Item = &'a str>, identifier: &str) -> bool {
    // iteriert durch Alle Personen/Cars und vergleicht deren ID. Bricht ab bei gleicher ID, sonst true.
    for per in liste {
        if identifier == per {
            return false
        }
    }
    true
}

// Findet den Index von der Person im übergebenen Vektor
fn get_person_index(liste: &Vec<Person>, identifier: &str) -> Option<usize> {
    // Wenn die identifiktation übereinstimmt wird direkt ein Option des Indexes zurück gegeben, sonst eben None. 
    for (idx, per) in liste.iter().enumerate() {
        if per.identifier == identifier {
            return Some(idx)
        }
    }
    None
}
// Kann später noch zu einer Funktion fusioniert werden. 
// Findet den Index von dem Car im übergebenen Vektor
fn get_car_index(liste: &Vec<Car>, identifier: &str) -> Option<usize> {
    // Wenn die identifiktation übereinstimmt wird direkt ein Option des Indexes zurück gegeben, sonst eben None. 
    for (idx, per) in liste.iter().enumerate() {
        if per.identifier == identifier {
            return Some(idx)
        }
    }
    None
}

// Da jeder nur ein Fahrzeug gleichzeitg ausleihen darf, prüfen wir hier ob schon bereits eins ausgeliehen wurde. 
fn has_no_car(liste: &Vec<(&str, &str)>, person_id: &str) -> bool {
    for rental in liste {
        let (per, _) = rental;
        if &person_id == per {
            // Er hat bereits ein Auto
            return false
        }
    }
    // Er hat kein Auto 
    true
}

// Da jedes Fahrzeug nur einer Person gleichzeitg ausleihen darf, prüfen wir hier ob es bereits verliehen wurde. 
fn has_no_person(liste: &Vec<(&str, &str)>, car_id: &str) -> bool {
    for rental in liste {
        let (_, car) = rental;
        if &car_id == car {
            // Er hat bereits eine Person. 
            return false
        }
        
    }
    // Er hat kein Person.
    true
}

// gibt true zurück wenn eine Reservierung besteht
fn reservated(liste: &Vec<Reservation>, person_id: &str, car_id: &str) -> bool {
    for r in liste {
        if r.person_id == person_id && r.car_id == car_id {
            // Es besteht bereits eine Reservierung der Art. 
            return true
        }
    }
    // Es besteht noch keine derartige Reservierung
    false
}

// Sortieralgorithmus nach Priorität
// sortiert vom Größten zum Kleinsten
fn bsort_priority(liste: &mut Vec<Reservation>) {
    let mut ch: bool = true; // gibt an ob ein Tausch vorgenommen wurde
    if liste.len() < 2 { ch = false; }
    // sortiere so lange bis keine Änderungen mehr gemacht wurden. 
    while ch {
        ch = false; 
        for idx in 0..liste.len() -1 {
            if liste[idx].priority < liste[idx+1].priority {
                liste.swap(idx, idx+1); // Tauscht die Riehenfolge und umgeht dabei das Ownership Problem mit den mutable Referenzen und ich darf es sogar verwenden da es eine Methode von Vektoren ist! Juhuuu!
                ch = true; 
            }
        }
    }
}

// gibt true wenn ein Fahrzeug ausgemustert werden muss. 
fn must_car_retire(age: u32, km: u32, rental_count: u32) -> bool {
    let s_age = age as f64 / MAX_AGE_DAYS as f64 ; 
    let s_km = km as f64 / MAX_KM as f64 ; 
    let s_rental_count = rental_count as f64 / MAX_RENTALS as f64 ; 
    let rs = s_age + s_km + s_rental_count ; 
    if rs >= 1.0 {
        true
    } else {
        false
    }
}
