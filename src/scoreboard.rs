use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerScore {
    pub name: String,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scoreboard {
    pub scores: HashMap<String, PlayerScore>,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard {
            scores: HashMap::new(),
        }
    }

    pub fn update_score(&mut self, player_name: &str, score: u32) {
        let entry = self.scores.entry(player_name.to_string()).or_insert(PlayerScore {
            name: player_name.to_string(),
            score: 0,
        });
        entry.score += score;
    }

    pub fn display(&self) {
        println!("Scoreboard:");
        for player in self.scores.values() {
            println!("{}: {}", player.name, player.score);
        }
    }
}
