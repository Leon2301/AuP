/*
 * Zur Idee:
 * https://en.wikipedia.org/wiki/Multiway_number_partitioning
 * Zu erst erstellen wir eine Team zuteilung nach dem greedy vorgehen. 
 * Davon berechnen wir die Summe des höchsten und niedrigsten Team.
 * Dann optimieren Wir so lange mit Backtracking bis wir keine besseren
 * Summen der Teams mehr erhalten. Dabei werden die Personen immer so 
 * lange in das nächste Team weiter geschoben, dass der Idealbereich 
 * eingehalten wird. 
 * Das ganze erweitern wir mit zusätzlichen Abbruchbedingungen für 
 * die effizeins.   
 * 
 * TODO: 
 * Wie baue ich das Maximal Gewicht mit ein ???
 */

use std::io::Write;
use std::fs::{read_to_string, File};
use std::u128;

#[derive(PartialEq, Debug, Clone)]
struct Person {
    first_name: String,
    surname: String,
    weight: u8,
    age: u8,
    size: u8,
}

#[derive(PartialEq, Debug, Clone)]
struct XRayP {
    score: u16,
    weight: u8,
    person: Person,
}

#[derive(PartialEq, Debug)]
enum TeamError {
    NotEnoughPeople,
    NotEverybodyInTeam,
}

trait SportsFestival<'a> {
    fn create_teams(
        &'a self,
        team_number: usize,
        strength_method: fn(&Person) -> u16,
        weight_threshold: u16,
    ) -> Result<Vec<Vec<&'a Person>>, TeamError>;

    fn read_participants(&mut self, path: &str) -> std::io::Result<()>;

    fn write_participants(&self, path: &str) -> std::io::Result<()>;
}

impl<'a> SportsFestival<'a> for Vec<Person> {
    fn create_teams(
        &'a self,
        team_number: usize,
        strength_method: fn(&Person) -> u16,
        weight_threshold: u16,
    ) -> Result<Vec<Vec<&'a Person>>, TeamError> {
        if self.len() < team_number { // testet ob es überhaupt eine Person pro Team gibt. 
            return Err(TeamError::NotEnoughPeople)
        }
        if team_number == 0 {
            if self.len() > 0 {
                return Err(TeamError::NotEverybodyInTeam)
            } else if self.len() == 0 {
                let result: Vec<Vec<&Person>> = Vec::new();
                return Ok(result)
            }
        }
        // Wir wissen also das es min ein Team gibt und auch min genauso viele Personen. 
        let mut dic_score_person: Vec<XRayP> = Vec::new(); // Speichert die Personen mit den Metadaten. Später verwendet man für sowas bestimmt HashMaps

        // Wir brauchen zunächst eine absteigend geordnete Liste von den Werten der Personen.
        for p in self.iter() {
            let score: u16 = strength_method(p);
            // Hier werden noch die Metadaten der Person in einen Vektor gepackt. 
            let x_ray_person = XRayP {
                score: score,
                weight: p.weight,
                person: p.clone(),
            };
            dic_score_person.push(x_ray_person);
        }
        dic_score_person.sort_by(|a, b| (b.score).cmp(&(a.score))); // sortiert absteigend

        let optimized_teams: Vec<Vec<XRayP>> = dic_score_person.optimize_greedy_algorithm(team_number, weight_threshold)?; // Hier passiert die wahre Magie.
        // konvergiert die Art "dictonary" wieder zurück in eine Referenz auf Personen. 
        let result_raw: Vec<Vec<&Person>> = optimized_teams
            .iter()
            .map(|a| a
                .iter()
                .map(|x| &x.person)
                .collect())
            .collect();

        // Es soll eine Referenz der anfänglichen Personen übergeben werden. Also bauen wir den Vektor nach, aus den Personen in Self, damit ich den Code nicht nochmal komplett umschreiben muss. 
        let mut result: Vec<Vec<&Person>> = Vec::new();
        for (team_idx,team) in result_raw.iter().enumerate() {
            for (idx,person) in team.iter().enumerate() {
                for p in self {
                    // Wie gut das PartialEq in der Aufgabenstellung derived wurde XD
                    if p == *person {
                        if idx == 0 {
                            result.push(vec![p]);
                        } else {
                            result[team_idx].push(p);
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    fn read_participants(&mut self, path: &str) -> std::io::Result<()> {
        // Datei einlesen
        let content: String = read_to_string(path)?;

        // In der Aufgabenselltung steht das wir keine Überprüfung für Doppelungen oder fehlerhafte Eingaben vornehmen müssen. 
        // Datei parsen
        for per in content.lines(){ // lines() macht geht die Datei Zeile für Zeile durch. 
            // Hier wird generell einige male zsichen &str und String umgewandelt. Es funktioniert ziemlich gut, wahrscheinlich kann man es aber noch effizienter gestallten. 
            // Prüfen ob das der String nicht leer ist: 
            if !per.is_empty()  {
                // Personen parsen
                let person = from_str(per)?;
                self.push(person);
            }
        }
        Ok(())
    }

    fn write_participants(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?; // create überschreibt praktischerweise auch vorhandene Dateien.
        for per in self {
            writeln!(
                file, 
                "\"{},{}\",{},{},{}", 
                per.surname, per.first_name, per.weight, per.age, per.size
            )?;
        }
        Ok(())
    }
}

// --- neue Methoden --- hier wirds wild

trait TeamCalculate {
    fn get_sum(&self) -> u128;
}
impl TeamCalculate for Vec<u16> {
    // gibt die Summe des Vektors
    fn get_sum(&self) -> u128 {
        let sum: u128 = self.iter().map(|&x| x as u128).sum::<u128>();
        sum
    }
}

trait TeamCalculateStruct {
    fn optimize_greedy_algorithm(&self, team_number: usize, weight_threshold: u16) -> Result<Vec<Vec<XRayP>>, TeamError>;
}

impl TeamCalculateStruct for Vec<XRayP> {
    // der greedy Ansatz ist im Laufe der Zeit wohl verloren gegangen... Wir stacken nun einfach immer weiter personen in ein Team bis eine Abbruchbedingung erreicht wird. Erst danach gehen wir ins nächste Team. Das ganze wird dann immer weiter optimiert, so das die Summe des höchsten Teams immer weiter sinkt und sich daduch auch die Differenz des höchsten zum niedrigsten Teams senkt. 
    fn optimize_greedy_algorithm(&self, team_number: usize, weight_threshold: u16) -> Result<Vec<Vec<XRayP>>, TeamError>{
        // Gehe sicher das Self absteigend ist
        let mut sorted_list: Vec<XRayP> = self.clone();
        sorted_list.sort_by(|a, b| (b.score).cmp(&(a.score)));// Sortiert absteigend. 

        // Hierkommt das ware Backtracking Zeug!
        // Hierbei nutzen wir den Trick das wir das Maximum immer kleiner werden lassen. Dadurch wird das Minimum automatisch höher und wir erhalten fairere Teams. 
        fn optimize_helper(
            current_teams: &mut Vec<Vec<XRayP>>,
            sorted_list: &Vec<XRayP>,
            team_number: usize,
            max_sum: u128, 
            weight_threshold: u16,
            diff_before: u128, // gibt die qualität der vorheriegen Lösung an
            idx: usize // Der Index von der aktuell zu zuordnenden Person. 
        ) -> bool {
            // Hier bauen wir uns aus den fancy X_Ray_P Vektoren -> u16 Vektoren.
            let current_teams_u16: Vec<Vec<u16>> = current_teams
                .iter()
                .map(|a| a
                    .iter()
                    .map(|x| x.score)
                    .collect())
                .collect();
            // Abbruchbedingungen
            if idx == sorted_list.len() {
                // Es wurden alle Leute zugeordnet
                // Ist in jedem Team mindestens eine Person?
                if current_teams_u16.len() == team_number {
                    // überprüfe ob die gefundene Lösung nicht die selbe ist, wie die davor.
                    if current_teams_u16.max_diff() < diff_before {
                        // bessere Lösung gefunden
                        return true;
                    } else {
                        // schlechtere Lösung...
                        return false;
                    }
                } else {
                    // es sind nicht genügend Teams erstellt worden. 
                    return false;
                }
            }
            // Überprüfen ob das Maximalgewicht überschritten wurde
            let max: Option<u16> = current_teams
                .iter()
                .map(|a| a
                    .iter()
                    .map(|x| x.weight as u16)
                    .sum::<u16>())
                .max();
            //let max_team: u16 = current_teams_total_weight.iter().copied().max().unwrap();
            if let Some(max_team) = max {
                if max_team > weight_threshold {
                    return false;
                }
            }
            if current_teams_u16.get_max_sum() > max_sum {
                // Diser "Pfad" ist bereits schlechter als die aktuelle Lösung. Daher nicht weiter testen. 
                return false;
            }
            // Ordne die aktuelle Person einem Team zu 
            for team_idx in 0..team_number {
                // Die ersten Personen müssen nicht auf jedes Team getestet werden. Wir wollen also isomorphe Varianten nicht testen. 
                if team_idx > idx { continue; }
                // Füge die aktuelle Person zum aktuellen Team hinzu
                let person: XRayP = sorted_list[idx].clone();
                let person_score: u16 = person.score;
                // Die Person zum current Ergebnis anhängen. 
                if team_idx < current_teams.len() {
                    current_teams[team_idx].push(person);
                } else {
                    current_teams.push(vec![person]);
                }
                // Schaue ob das zu einem besseren Ergebnis führt, sonst entferne die Person wieder aus diesem Team. 
                if optimize_helper(current_teams, sorted_list, team_number, max_sum, weight_threshold, diff_before, idx+1) {
                    return true;
                } else {
                    current_teams[team_idx].pop();
                    // Effektiv ist nun nichts passiert und wir probieren die Person ins nächste Team zu stecken. 
                }
            }
            false            
        }
        
        // Dieser vorgang soll so lange optimiert werden, bis keine bessere Lösung mehr gefunden wurde.
        let mut bedder_solution_found: bool = true; 
        // TODO: Teste vorher mit dem Greedy Algorithm ob es eine gute Lösung gibt und optimiere diese.
        let mut teams: Option<Vec<Vec<XRayP>>> = None;
        while bedder_solution_found {
            // Überprüft ob es bereits eine Lösung zum optimieren gibt. Ansonsten fängt er von Vorne an. 
            let max_sum: u128;
            let diff_before: u128;
            if let Some(ref raw) = teams {
                let current_teams_u16: Vec<Vec<u16>> = raw.iter().map(|a| a.iter().map(|x| x.score).collect()).collect();
                max_sum = current_teams_u16.get_max_sum();
                diff_before = current_teams_u16.max_diff();
            } else {
                max_sum = u128::MAX;
                diff_before = u128::MAX;
            }
            
            let mut new_teams: Vec<Vec<XRayP>> = Vec::with_capacity(team_number);
            bedder_solution_found = optimize_helper(&mut new_teams, &sorted_list, team_number, max_sum, weight_threshold, diff_before, 0usize); // optimieren

            if bedder_solution_found {
                // wir wollen unser vorheriges Ergebniss wirklich nur überschreiben, wenn wir auch ein besseres gefunden haben. 
                teams = Some(new_teams);
            }
        }
        // Überprüfen ob wir überhaupt eine Lösung haben. 
        if let Some(raw) = teams {
            return Ok(raw);
        } else {
            return Err(TeamError::NotEverybodyInTeam);
        }
    }
}

trait TeamMatcher {
    fn max_diff(&self) -> u128;
    fn get_max_sum(&self) -> u128;
}
impl TeamMatcher for Vec<Vec<u16>> {
    // Berechnet die maximale Differenz der Summen von den Teams
    fn max_diff(&self) -> u128 {
        // Ist blöd das so zu machen, da die Summen der Teams damit auf ein Maximum beschränkt werden. Mir ist jedoch noch kein besserer Weg eingefallen die Differenz zu berchnen. 
        let mut max_value: u128 = 0; 
        let mut min_value: u128 = u128::MAX;

        for t in self {
            if t.get_sum() < min_value { min_value = t.get_sum(); }
            if t.get_sum() > max_value { max_value = t.get_sum(); }
        }
        max_value - min_value
    }
    // gibt die höchste Summe der Teams 
    fn get_max_sum(&self) -> u128 {
        let mut max_sum: u128 = 0; 
        for team in self {
            if team.get_sum() > max_sum {
                max_sum = team.get_sum();
            }
        }
        max_sum
    }
}

// --- Hilfsfunktionen ---

// wandelt einen &str in eine Person verpackt in einem Result. 
fn from_str(s: &str) -> std::io::Result<Person> { // Da der Fehler mir hier egal ist, lasse ich den genauen Fehler hier einfach verfallen.
    // s sollte die Form haben: 
    // "Nachname, Vorname", Gewicht, Alter, Größe
    let clean: String = s.replace('\"',"").replace("\r",""); // um sicher zu gehen.
    let parts: Vec<&str> = clean.split(',').collect::<Vec<&str>>(); // Seperiert den Namen und Zahlen
    Ok(Person {
        first_name: parts[1].parse().unwrap(),
        surname: parts[0].parse().unwrap(),
        weight: parts[2].parse().unwrap(), // Da wir laut Aufgabenstellung sicher korrekte Daten bekommen, entpacken wir hier direkt. Andernfalls könnte es hier schnell zu einer Panic kommen. 
        age: parts[3].parse().unwrap(),
        size: parts[4].parse().unwrap(),
    })
}
