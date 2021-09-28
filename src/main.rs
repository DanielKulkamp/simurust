use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
use rand::Rng;


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
                self.points += 1;
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
                self.points += 1;
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
        let home_score : i32 = rand::thread_rng().gen_range(0..4);
        let away_score : i32 = rand::thread_rng().gen_range(0..4);
        (home_score, away_score)
    }



    // fn run_match(home: &mut Team, away: &mut Team, home_score: i32, away_score: i32 ){
    //     home.games += 1;
    //     home.goals_for += home_score;
    //     home.goals_against += away_score;
    //     home.goal_difference += home_score - away_score;
    //     away.games += 1;
    //     away.goals_for += away_score;
    //     away.goals_against += home_score;
    //     away.goal_difference += away_score - home_score;
    //     let w : f32;
    //     match home_score.cmp(&away_score) {
    //         Ordering::Less => {
    //             home.losses += 1;
    //             home.home_losses += 1;
    //             away.wins += 1;
    //             away.away_wins += 1;
    //             away.points += 3;
    //             w = 0.0;
    //         },
    //         Ordering::Equal => {
    //             home.points += 1;
    //             home.draws += 1;
    //             home.home_draws +=1;
    //             away.draws += 1;
    //             away.away_draws += 1;
    //             away.points += 1;
    //             w = 0.5;

    //         },
    //         Ordering::Greater => {
    //             home.points += 3;
    //             home.wins += 1;
    //             home.home_wins += 1;
    //             away.losses += 1;
    //             away.away_losses += 1;
    //             w = 1.0;
    //         }
    //     }
    //     let dr = home.rating - away.rating;
    //     let we : f32 = 1.0/(1.0+10f32.powf(-dr/400.0));
    //     let i = 60f32;
    //     let delta = i * (w-we);
    //     home.rating += delta;
    //     away.rating -= delta;
    // }
}

fn main() {
    let conteudo = fs::read_to_string("C:\\Users\\danie\\Simulador Serie B\\SimuladorPontosCorridos\\TABELA SERIE B.txt").expect("Erro");
    let team_names = vec!["CFC", "BOT", "AVA", "CRB", "GOI", "GUA", "CSA", "VAS", "REM", "SAM", "PON", "OPE", "NAU", "CRU", "VIL", "LON",
        "VIT", "BRU", "CON", "BRA"];
    
        let mut teams_map  :  HashMap<&str, Team> = HashMap::new();    
    for t in team_names.iter() {
        teams_map.insert(t, Team::create(t.to_string()) );
    }
    
    for line in conteudo.lines(){
        let v : Vec<&str> = line.split(' ').collect();
        let home : &Team;
        let away : &Team;
        home = match teams_map.get(v[0]){
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
        
        if v[1] != "-" {
            let home_score : i32;
            let away_score : i32;
            let mut iter = v[1].chars();

            home_score = iter.next().unwrap().to_digit(10u32).unwrap() as i32;
            println!("Home score: {}", home_score);
            iter.next();
            away_score = iter.next().unwrap().to_digit(10u32).unwrap() as i32;

            let w : f32 = match home_score.cmp(&away_score) {
                Ordering::Greater => 1.0f32,
                Ordering::Equal => 0.5f32,
                Ordering::Less => 0.0f32
            };

            let delta_rating = Team::calc_delta_rating(home, away, w);

            {
                let homet : &mut Team = teams_map.get_mut(v[0]).unwrap();
                homet.home_match(home_score, away_score, delta_rating);
                println!("{:?}", homet);
                
            }

            {
                let awayt : &mut Team = teams_map.get_mut(v[2]).unwrap();
                awayt.away_match(home_score, away_score, delta_rating);
                println!("{:?}", awayt);

            }

        }

        //onana
        
    
    }

    
}




#[test]
fn home_wins() {
    let mut home = crate::Team::create(String::from("VAS"));
    let mut away = crate::Team::create(String::from("GUA"));
    crate::Team::run_match(&mut home, &mut away, 3, 1);
    assert_eq!(home.goals_for, 3);
    assert_eq!(home.goals_against, 1);
    assert_eq!(home.goal_difference, 2);
    assert_eq!(home.points, 3);
    assert_eq!(home.wins, 1);
    assert_eq!(home.home_wins, 1);
     assert_ne!(home.rating, 1000f32);
}

#[test]
fn away_wins() {
    let mut home = crate::Team::create(String::from("VAS"));
    let mut away = crate::Team::create(String::from("GUA"));
    crate::Team::run_match(&mut home, &mut away, 1, 3);
    assert_eq!(away.goals_for, 3);
    assert_eq!(away.goals_against, 1);
    assert_eq!(away.goal_difference, 2);
    assert_eq!(away.points, 3);
    assert_eq!(away.wins, 1);
    assert_eq!(away.away_wins, 1);
    assert_ne!(home.rating, 1000f32);
}
