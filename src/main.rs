use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
use rand::{thread_rng, Rng};


#[derive(Debug)]
struct Team {
    name: String,
    games: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    goals_for: i32,
    goals_against: i32,
    goal_difference: i32,
    home_wins: i32,
    home_draws: i32,
    home_losses: i32,
    away_wins: i32,
    away_draws: i32,
    away_losses: i32,
    rating: f32,
    points: i32
}

impl Team {
    fn create(name: String) -> Team {
        Team {
            name: name,
            games: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0,
            goal_difference: 0,
            home_wins: 0,
            home_draws: 0,
            home_losses: 0,
            away_wins: 0,
            away_draws: 0,
            away_losses: 0,
            rating: 1000.0,
            points: 0
        }
    }

    fn home_match(&mut self, home_score: i32, away_score : i32 , delta_rating: f32 ){
        self.games += 1;
        self.goals_for += home_score;
        self.goals_against += away_score;
        self.goal_difference += home_score - away_score;
        self.rating += delta_rating;
        match home_score.cmp(&away_score) {
            Ordering::Less => {
                self.losses += 1;
                self.home_losses +=1;
            },
            Ordering::Equal => {
                self.draws += 1;
                self.points += 1;
                self.home_draws += 1;
            },
            Ordering::Greater => {
                self.wins += 1;
                self.home_wins += 1;
                self.points += 3;
            }
        }
    }

    fn away_match(&mut self, home_score: i32, away_score : i32, delta_rating: f32){
        self.games += 1;
        self.goals_for += away_score;
        self.goals_against += home_score;
        self.goal_difference -= home_score - away_score;
        self.rating -= delta_rating;
        match away_score.cmp(&home_score) {
            Ordering::Less => {
                self.losses += 1;
                self.away_losses += 1;
            },
            Ordering::Equal => {
                self.draws += 1;
                self.points += 1;
                self.away_draws += 1;
            },
            Ordering::Greater => {
                self.wins += 1;
                self.away_wins += 1;
                self.points += 3;
            }
        }
    }

    fn calc_delta_rating( home: &Team, away: &Team, w: f32) -> f32 {
        const IMPORTANCE : f32 = 60.0f32;
        const HOME_BONUS : f32 = 100.0f32;
        let d_r = home.rating + HOME_BONUS - away.rating;
        let w_e = 1.0f32/(1.0+10.0f32.powf(-d_r/400.0f32));
        return IMPORTANCE*(w -w_e); 
    }

    fn sim_match_rand(_home: &Team, _away: &Team) -> (i32, i32){ 
        let home_score : i32 = thread_rng().gen_range(0..4);
        let away_score : i32 = thread_rng().gen_range(0..4);
        (home_score, away_score)
    }

    fn sim_match_ved(home: &Team, away: &Team) -> (i32, i32){
        let total_home = home.home_wins+home.home_draws+home.home_losses;
        let total_away = away.away_wins+away.away_draws+away.away_losses;
        let mut key :i32 = rand::thread_rng().gen_range(0..(total_home+total_away));
        if key < home.home_wins + away.away_losses {
            return (1 , 0);
        }
        key -= home.home_wins + away.away_losses;
        if key < home.home_draws + away.away_draws {
            return (0,0);
        }
        (0, 1)
        
    }

    fn sim_match_ELO(home: &Team, away: &Team) -> (i32, i32){
        let w_e: f32 =  1.0/(1.0+10.0f32.powf((away.rating-home.rating)/400.0));
        let w_a = 1.0 - w_e;
        let w_d : f32;
        if w_e >= 0.5 {
            w_d = w_a;
        } else {
            w_d = w_e;
        }
        let res = thread_rng().gen_range(0f32..w_e+w_a+w_d);
        if res < w_e {
            return (1, 0);
        }
        if res - w_e < w_d {
            return (0, 0);
        }
        (0, 1)
    }

    
}

fn main() {
    let algoritmo = Team::sim_match_ELO;
    let n_simulacoes = 100000u32;

    let conteudo = fs::read_to_string("C:\\Users\\danie\\Simulador Serie B\\SimuladorPontosCorridos\\TABELA SERIE B.txt").expect("Erro");
    let mut team_names = vec!["CFC", "BOT", "AVA", "CRB", "GOI", "GUA", "CSA", "VAS", "REM", "SAM", "PON", "OPE", "NAU", "CRU", "VIL", "LON",
        "VIT", "BRU", "CON", "BRA"];
    
    let mut promotion_map : HashMap<&str, i32> = HashMap::new();
    let mut title_map : HashMap<&str, i32> = HashMap::new();
    let mut relegation_map : HashMap<&str, i32> = HashMap::new();

    for t in team_names.iter() {
        promotion_map.insert(t, 0);
        title_map.insert(t, 0);
        relegation_map.insert(t, 0);
    }

    let mut title_points : HashMap<i32, i32> = HashMap::new();
    let mut promotion_points : HashMap<i32, i32> = HashMap::new();
    let mut relegation_points : HashMap<i32, i32> = HashMap::new();

    for i in 0..100 {
        title_points.insert(i, 0);
        promotion_points.insert(i, 0);
        relegation_points.insert(i, 0);
    }


    print!("▓");
    for _i in 0..n_simulacoes {
        
        let mut teams_map  :  HashMap<&str, Team> = HashMap::new();    
        for t in team_names.iter() {
            teams_map.insert(t, Team::create(t.to_string()) );
        }

        for line in conteudo.lines(){
            let v : Vec<&str> = line.split(' ').collect();
            let home : &Team;
            let away : &Team;
            home = match teams_map.get(v[0]) {
                Some(team) => team,
                None => {
                    panic!("erro time não existe");
                }
            };
                        
            away = match teams_map.get(v[2]){
                Some(team) => team,
                None => {
                    panic!("erro time não existe");
                }
            };
            let home_score : i32;
            let away_score : i32;
            if v[1] != "-" {
                let mut iter = v[1].chars();
                home_score = iter.next().unwrap().to_digit(10u32).unwrap() as i32;
                iter.next();
                away_score = iter.next().unwrap().to_digit(10u32).unwrap() as i32;
            } else {
                
                let (h_s, a_s) = algoritmo(home, away);
                home_score = h_s;
                away_score = a_s;
                
            }

            let w : f32 = match home_score.cmp(&away_score) {
                Ordering::Greater => 1.0f32,
                Ordering::Equal => 0.5f32,
                Ordering::Less => 0.0f32
            };

            let delta_rating = Team::calc_delta_rating(home, away, w);

            {
                let homet : &mut Team = teams_map.get_mut(v[0]).unwrap();
                homet.home_match(home_score, away_score, delta_rating);
                             
            }

            {
                let awayt : &mut Team = teams_map.get_mut(v[2]).unwrap();
                awayt.away_match(home_score, away_score, delta_rating);
                
            }

           
            
        }
        team_names.sort_by_key(|a| -1*teams_map.get(a).unwrap().goal_difference);
        team_names.sort_by_key(|a| -1*teams_map.get(a).unwrap().wins);
        team_names.sort_by_key(|a| -1*teams_map.get(a).unwrap().points);

        for t in team_names[0..4].iter(){
            *promotion_map.entry(t).or_insert(0) += 1;            
        }
        *title_map.entry(team_names[0]).or_insert(0) +=1;
        for t in team_names[16..20].iter(){
            *relegation_map.entry(t).or_insert(0) += 1;            
        }
        for i in teams_map.get(team_names[3]).unwrap().points..100 {
            *promotion_points.entry(i).or_insert(0) += 1;
        }
        for i in teams_map.get(team_names[0]).unwrap().points..100 {
            *title_points.entry(i).or_insert(0) += 1;
        }
        for i in 0..teams_map.get(team_names[16]).unwrap().points {
            *relegation_points.entry(i).or_insert(0) += 1;
        }

    }
    team_names.sort_by_key(|a| relegation_map.get(a).unwrap());
    team_names.sort_by_key(|a| promotion_map.get(a).unwrap()*-1);
    team_names.sort_by_key(|a| title_map.get(a).unwrap()*-1);
    println!("\nTime\tTitulo\tAcesso\tDescenso");
    for team in team_names.iter() {
        let title : f32 = 100.0 * (*title_map.get(team).unwrap() as f32 )/ (n_simulacoes as f32);
        let promotion : f32 = 100.0 * (*promotion_map.get(team).unwrap() as f32) / (n_simulacoes as f32);
        let relegation : f32 = 100.0 * (*relegation_map.get(team).unwrap() as f32) / (n_simulacoes as f32);
        println!("{}\t{:>5.2}\t{:>5.2}\t{:>5.2}",team, title, promotion, relegation);
    }

    println!("\nPor Pontuação:");
    for i in 16..90 {
        println!("{}: Titulo: {:>5.2}, \tAcesso: {:>5.2}\tQueda: {:>5.2}", 
        i, *title_points.get(&i).unwrap() as f32 / n_simulacoes as f32 * 100.0,
        *promotion_points.get(&i).unwrap() as f32 / n_simulacoes as f32 * 100.0, 
        *relegation_points.get(&i).unwrap() as f32 / n_simulacoes as f32 * 100.0);
    }

    
}

