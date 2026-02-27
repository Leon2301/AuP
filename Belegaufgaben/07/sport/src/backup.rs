#!(allow(dead_code))

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

//use std::io::Result;
use std::io::Write;
use std::fs::{read_to_string, File};
use std::panic::resume_unwind;
use std::{result, u128};
use std::str::FromStr;
use std::cmp::max;

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
            } else {
                let result: Vec<Vec<&Person>> = Vec::new();
                return Ok(result)
            }
        }
        // Wir wissen also das es min ein Team gibt und auch min genauso viele Personen. 
        //let mut score_list: Vec<u16> = Vec::new();
        let mut dic_score_person: Vec<XRayP> = Vec::new();

        // Wir brauchen zunächst eine absteigend geordnete Liste von den Werten der Personen.
        for p in self.iter() {
            let score = strength_method(p);
            //score_list.push(score);
            // Hier werden noch die Metadaten der Person in einen Vektor gepackt. 
            let x_ray_person = XRayP {
                score: score,
                weight: p.weight,
                person: p.clone(),
            };
            dic_score_person.push(x_ray_person);
        }
        //score_list.sort_by(|a, b| b.cmp(a)); // Sortiert absteigend. 
        dic_score_person.sort_by(|a, b| (b.score).cmp(&(a.score)));
        //print!("{:?}; ", score_list);
        print!("{:?}; ", dic_score_person);

        // Die erste Lösung erstellen mit dem greedy Algorithm
        //let first_teams: Vec<Vec<u16>> = score_list.greedy_algorithm(team_number)?;
        //println!("{:?}; ", score_list);
        //println!("first_teams:     {:?}; ", first_teams);
        //println!("tems scores:     {:?}; ", first_teams.get_every_team_sum());

        // TODO: Beachte den Fall das der greedy Algorithm bereits keine Lösung findet. 

        // Optimiere nur, falls die Chance besteht eine bessere Lösung zu finden.
        //let optimized_teams: Vec<Vec<u16>>;
        //if first_teams.max_diff() != 0 {
        //    optimized_teams = score_list.optimize_greedy_algorithm(team_number, first_teams, weight_threshold)?;
        //} else {
        //    optimized_teams = first_teams;
        //}
        let optimized_teams = dic_score_person.optimize_greedy_algorithm(team_number, weight_threshold)?;
        let result_raw: Vec<Vec<&Person>> = optimized_teams.iter().map(|a| a.iter().map(|x| &x.person).collect()).collect();

        let current_teams_u16: Vec<Vec<u16>> = optimized_teams.iter().map(|a| a.iter().map(|x| x.score).collect()).collect();
        println!("optimized_teams: {:?}; ", optimized_teams);
        println!("optimized_teams: {:?}; ", current_teams_u16);
        //println!("tems scores:     {:?}; ", optimized_teams.get_every_team_sum());

        // Überprüfe ob das maximal Gewicht überschritten wurde. 

        //let teams: Vec<Vec<u16>> = match_teams(team_number, score_list, weight_threshold); //   MUSS NOCH EIN RESULT ZURÜCKGEBEN:

        // Es soll eine Referenz der anfänglich Personen übergeben werden. Also bauen wir den Vektor nach aus den Personen in Self, damit ich den Code nicht nochmal komplett umschreiben muss. 
        let mut result: Vec<Vec<&Person>> = Vec::new();
        for (team_idx, team) in result_raw.iter().enumerate() {
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
            // Prüfen ob der String nicht leer ist: 
            if !per.is_empty()  {
                // Personen parsen
                let person = from_str(per)?;
                self.push(person);
            }
        }
        //println!("Self: {:?}", self);
        Ok(())
    }

    fn write_participants(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        //file.truncate(true); // Setze die Datei auf Länge 0. das macht create schon
        for per in self {
            //file?.write_all(b"Hallo ihr dicken!")?;
            writeln!(
                file, 
                "\"{},{}\",{},{},{}", 
                per.surname, per.first_name, per.weight, per.age, per.size
            )?;
        }
        Ok(())
        // TODO
    }
}

// --- neue Methoden ---

trait TeamCalculate {
    fn get_sum(&self) -> u128;
    fn average_sum_per_team(&self, team_number: usize) -> f64;
    fn is_smaler(&self, max: u128) -> bool;
    fn is_valid(&self, min: u128, max: u128) -> bool;
    fn greedy_algorithm(&self, team_number: usize) -> Result<Vec<Vec<u16>>, TeamError>;
    fn optimize_greedy_algorithm(&self, team_number: usize, current_solution: Vec<Vec<u16>>, weight_threshold: u16) -> Result<Vec<Vec<u16>>, TeamError>;
}
impl TeamCalculate for Vec<u16> {
    // gibt die Summe des Vektors
    fn get_sum(&self) -> u128 {
        let sum: u128 = self.iter().map(|&x| x as u128).sum::<u128>();
        sum
    }
    // Durchschnittssumme pro Team
    fn average_sum_per_team(&self, team_number: usize) -> f64 {
        let sum: f64 = self.get_sum() as f64;
        let result: f64 = sum / team_number as f64;
        result
    }
    // gibt true wenn die Summe kleiner als ein gegebenes Maximum ist
    fn is_smaler(&self, max: u128) -> bool {
        let sum: u128 = self.get_sum();
        if sum < max {
            return true
        }
        false
    }
    // gibt true wenn die Summe im vorgegebenen Intervall liegt
    fn is_valid(&self, min: u128, max: u128) -> bool {
        // wir nehmen hier ein .iter() damit wir es noch auf u128 casten. Wenn ich ein .into_iter() verwenden würde könnte ich zwar das .map weglassen, dann würde ich das Ergebnis allerdings in einem u16 lassen müssen. 
        let sum: u128 = self.get_sum();
        if min < sum && sum < max {
            return true
        }
        false
    }
    // Der greedy Algorithm findet eine gute Lösung die wir dann weiter optimieren können. 
    fn greedy_algorithm(&self, team_number: usize) -> Result<Vec<Vec<u16>>, TeamError> {
        // Gehe sicher das Self absteigend ist
        let mut sorted_list: Vec<u16> = self.clone();
        sorted_list.sort_by(|a, b| b.cmp(a)); // Sortiert absteigend. 

        let mut teams: Vec<Vec<u16>> = Vec::with_capacity(team_number);

        // ordnet der Reihe nach sorted_list den teams zu, bis es keine Personen mehr in sorted_list gibt. 
        for (idx, el) in sorted_list.into_iter().enumerate() {
            // in Jedes Team erstmal einen reinstcken
            if idx < team_number {
                teams.push(vec![el]);
            } else {
                let pos = idx % team_number; // gibt den Index von dem Team.
                teams[team_number-1 - pos].push(el);
            }
        }

        //Err(TeamError::NotEnoughPeople)
        Ok(teams)
    }
    fn optimize_greedy_algorithm(&self, team_number: usize, current_solution: Vec<Vec<u16>>, weight_threshold: u16) -> Result<Vec<Vec<u16>>, TeamError>{
        // Gehe sicher das Self absteigend ist
        let mut sorted_list: Vec<u16> = self.clone();
        sorted_list.sort_by(|a, b| b.cmp(a)); // Sortiert absteigend. 

        let mut teams: Vec<Vec<u16>> = current_solution.clone();

        // Hierkommt das ware Backtracking Zeug!
        // Hierbei nutzen wir den Trick das wir das Maximum immer kleiner werden lassen. Dadurch wird das Minimum automatisch höher und wir erhalten fairere Teams. 
        fn optimize_helper(
            current_teams: &mut Vec<Vec<u16>>,
            sorted_list: &Vec<u16>,
            team_number: usize,
            max_sum: u128, 
            weight_threshold: u16,
            diff_before: u128, // gibt die qualität der vorheriegen Lösung an
            idx: usize // Der Index von der aktuell zu zuordnenden Person. 
        ) -> bool {
            // Abbruchbedingungne
            if idx == sorted_list.len() {
                println!("keine Leute mehr, teams: {:?}", current_teams);
                // alle Personen in Teams geordnet
                // überprüfe ob die gefundene Lösung nicht die selbe ist, wie die davor.
                if current_teams.max_diff() < diff_before {
                    // bessere Lösung gefunden
                    return true;
                } else {
                    // schlechtere Lösung...
                    return false;
                }
                // TODO
            }
            // TODO: Überprüfen ob das Maximalgewicht überschritten wurde
            
            if current_teams.get_max_sum() > max_sum {
                // Diser "Pfad" ist bereits schlechter als die aktuelle Lösung. Daher nicht weiter testen. 
                return false;
            }
            // Ordne die aktuelle Person einem Team zu 
            for team_idx in 0..team_number {
                // Die ersten Personen müssen nicht auf jedes Team getestet werden. Wir wollen also isomorphe Varianten nicht testen. 
                if team_idx > idx { continue; }
                // Füge die aktuelle Person zum aktuellen Team hinzu
                let person: u16 = sorted_list[idx];
                current_teams.push_element(person, team_idx);
                print!("{}, {}; ", team_idx, sorted_list[idx]);
                println!("{:?} , max_sum: {}", current_teams, max_sum);
                // Schaue ob das zu einem besseren Ergebnis führt, sonst entferne die Person wieder aus diesem Team. 
                if optimize_helper(current_teams, sorted_list, team_number, max_sum, weight_threshold, diff_before, idx+1) {
                    return true;
                } else {
                    current_teams[team_idx].pop();
                    // Effektiv ist nun nichts passiert und wir probieren die Person ins nächste Team zu stecken. 
                }
            }

            // TODO: Überprüfe das Maximalgewicht und schaue ob der Fehler: auftritt. return Err(TeamError::NotEverybodyInTeam);

            false            
        }
        
        // Dieser vorgang soll so lange optimiert werden, bis keine bessere Lösung mehr gefunden wurde.
        let mut bedder_solution_found: bool = true; 
        while bedder_solution_found {
            let max_sum: u128 = teams.get_max_sum();
            let diff_before: u128 = teams.max_diff();
            let mut new_teams: Vec<Vec<u16>> = Vec::with_capacity(team_number);
            bedder_solution_found = optimize_helper(&mut new_teams, &sorted_list, team_number, max_sum, weight_threshold, diff_before, 0usize);

            println!("Besseres Ergebnis gefunden? {}", bedder_solution_found);
            if bedder_solution_found {
                // wir wollen unser vorheriges Ergebniss wirklich nur überschreiben, wenn wir auch ein besseres gefunden haben. 
                teams = new_teams;
            }
        }
        Ok(teams)
    }
}

trait TeamCalculateStruct {
    fn optimize_greedy_algorithm(&self, team_number: usize, weight_threshold: u16) -> Result<Vec<Vec<XRayP>>, TeamError>;
}

impl TeamCalculateStruct for Vec<XRayP> {
    fn optimize_greedy_algorithm(&self, team_number: usize, weight_threshold: u16) -> Result<Vec<Vec<XRayP>>, TeamError>{
        // Gehe sicher das Self absteigend ist
        let mut sorted_list: Vec<XRayP> = self.clone();
        sorted_list.sort_by(|a, b| (b.score).cmp(&(a.score)));// Sortiert absteigend. 

        //let mut teams: Vec<Vec<u16>> = current_solution.clone();

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
            // Hier bauen wir uns fancy aus den X_Ray_P Vektoren -> u16 Vektoren.
            let current_teams_u16: Vec<Vec<u16>> = current_teams.iter().map(|a| a.iter().map(|x| x.score).collect()).collect();
            println!("current_teams_u16: {:?}", current_teams_u16);
            // Abbruchbedingungne
            if idx == sorted_list.len() {
                println!("Alle Leute zugeordnet. {} < {}", current_teams_u16.max_diff(), diff_before);
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
                print!("{}, {:?}; ", team_idx, person_score);
                println!("{:?} , max_sum: {}", current_teams_u16, max_sum);
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
        // TODO: Teste vorher mit dem Greedy Algorithm ob es eine Lösung gibt. 
        let mut teams: Option<Vec<Vec<XRayP>>> = None;
        while bedder_solution_found {
            // Überprüft ob es bereits eine Lösung zum optimieren gibt. Ansonsten fängt er von Forne an. 
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
            bedder_solution_found = optimize_helper(&mut new_teams, &sorted_list, team_number, max_sum, weight_threshold, diff_before, 0usize);

            println!("Besseres Ergebnis gefunden? {}", bedder_solution_found);
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
    fn get_min_sum(&self) -> u128;
    fn get_every_team_sum(&self) -> Vec<u128>;
    fn push_element(&mut self, el: u16, idx: usize);
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
    // gibt die niedrigste Summe der Teams 
    fn get_min_sum(&self) -> u128 {
        let mut min_sum: u128 = 0; 
        for team in self {
            if team.get_sum() < min_sum {
                min_sum = team.get_sum();
            }
        }
        min_sum
    }
    // berechnet einen Vektor der die Summe des scores der einzelnen Teams enthält
    fn get_every_team_sum(&self) -> Vec<u128> {
        let mut result: Vec<u128> = Vec::new();
        for t in self {
            let sum = t.get_sum();
            result.push(sum);
        }
        result
    }
    // Schaut ob es an den verschachtelten Vektor angehängt werden muss oder der vektor erweitert wird. 
    fn push_element(&mut self, el: u16, idx: usize) {
        if idx < self.len() {
            self[idx].push(el);
        } else {
            self.push(vec![el]);
        }
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

fn match_teams(
    team_number: usize, 
    score_list: Vec<u16>,
    weight_threshold: u16
) -> Vec<Vec<u16>> { // noch in ein Result packen. 
    let average = score_list.iter().sum::<u16>() as f64 / team_number as f64; // berechnet den Durchschnittswert den jedes Team im Idealfall haben sollte. 
    println!("{} ", average);

    // Da die Reihenfolge der Teams keine Rolle spielt, packen wir den größten einfach immer in das nullte Team. 
    let mut result: Vec<Vec<u16>> = vec![vec![score_list[0]]];
    //match_teams_helper(
    //    &mut result,
    //    result.clone(),
    //    team_number,
    //    score_list,
    //    1,
    //    weight_threshold
    //);

    print!("{:?}; ", result);
    result
}

fn match_teams_helper(
    teams: &mut Vec<Vec<u16>>,
    current_teams: Vec<Vec<u16>>,
    team_number: usize, 
    score_list: Vec<u16>,
    current_score_idx: usize,
    weight_threshold: u16,
) -> u128 {
    0
}

// LÖSCHEN!!!

fn test_weight(p: &Person) -> u16 { p.weight as u16 }
fn test_mix(p: &Person) -> u16 { p.weight as u16 + p.size as u16}

fn main() {
    println!("--- Sportfest 2026 ---");
    let mut teilnehmer = Vec::<Person>::new();
    let _ = teilnehmer.read_participants(r"C:\Users\leonm\OneDrive\Mathematik\Code\AuP\Belegaufgaben\07\test_personen.csv");
    let _ = teilnehmer.read_participants(r"C:\Users\leonm\OneDrive\Mathematik\Code\AuP\Belegaufgaben\07\test_personen2.csv");
    
    let _ = teilnehmer.write_participants(r"C:\Users\leonm\OneDrive\Mathematik\Code\AuP\Belegaufgaben\07\test_output.csv");

    println!("\n-> Teams: \n {:?}", 
        teilnehmer.create_teams(
            2,
            test_mix,
            200
        )
    );

    println!("-> Teilnehmer: \n {:?}", teilnehmer);
}
/* 
 * 
 *  //let name: Vec<&str> = parts[0].split(',').collect(); // Separiert Vor- und Nachnamen
    println!("parts: {:?}", parts);
    println!("{:?}", Person {
        first_name: parts[1].to_string(),
        surname: parts[0].to_string(),
        weight: parts[2].parse().unwrap(), // Da wir laut Aufgabenstellung sicher korrekte Daten bekommen, entpacken wir hier direkt. Andernfalls könnte es hier schnell zu einer Panic kommen. 
        age: parts[3].parse().unwrap(),
        size: parts[4].parse().unwrap(),
    }); */