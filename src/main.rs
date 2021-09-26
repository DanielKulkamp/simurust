use std::cmp::Ordering;

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

    fn run_match(home: &mut Team, away: &mut Team, home_score: i32, away_score: i32 ){
        home.games += 1;
        home.goals_for += home_score;
        home.goals_against += away_score;
        home.goal_difference += home_score - away_score;
        away.games += 1;
        away.goals_for += away_score;
        away.goals_against += home_score;
        away.goal_difference += away_score - home_score;
        let w : f32;
        match home_score.cmp(&away_score) {
            Ordering::Less => {
                home.losses += 1;
                home.home_losses += 1;
                away.wins += 1;
                away.away_wins += 1;
                away.points += 3;
                w = 0.0;
            },
            Ordering::Equal => {
                home.points += 1;
                home.draws += 1;
                home.home_draws +=1;
                away.draws += 1;
                away.away_draws += 1;
                away.points += 1;
                w = 0.5;

            },
            Ordering::Greater => {
                home.points += 3;
                home.wins += 1;
                home.home_wins += 1;
                away.losses += 1;
                away.away_losses += 1;
                w = 1.0;
            }
        }
        let dr = home.rating - away.rating;
        let we : f32 = 1.0/(1.0+10f32.powf(-dr/400.0));
        let i = 60f32;
        let delta = i * (w-we);
        home.rating += delta;
        away.rating -= delta;
    }
}

fn main() {
    let a = Team::create(String::from("VAS"));
    println!("{:?}", a);
    println!("Hello, world!");
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
