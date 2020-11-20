use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::get;

#[derive(Deserialize)]
pub struct AdventureData {
    pub done: bool,
    pub time_left: i64,
    pub number: u8,
}

#[derive(Deserialize)]
pub struct ProfileData {
    pub character_name: String,
    pub level: u8,
    pub race: String,
    pub class: Vec<String>,
    pub adventure: Option<AdventureData>,
}

impl ProfileData {
    pub fn get_state(&self) -> String {
        match &self.adventure {
            Some(adv) => {
                if adv.done {
                    format!("On adventure {} (finished)", adv.number)
                } else {
                    format!("On adventure {}", adv.number)
                }
            }
            None => String::from("Idling"),
        }
    }
    pub fn get_time(&self) -> Option<(i64, i64)> {
        if let Some(adv) = &self.adventure {
            if adv.time_left > 0 {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                let then = now + adv.time_left;
                Some((now, then))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_details(&self) -> String {
        format!("Level {} {}", self.level, self.race)
    }
    pub fn get_small_image(&self) -> &str {
        get_class_image(&self.class[0])
    }
    pub fn get_small_image_text(&self) -> String {
        format!("{}/{}", self.class[0], self.class[1])
    }
    pub fn get_big_image_text(&self) -> String {
        self.character_name.clone()
    }
}

pub fn get_profile(user_id: i64) -> ProfileData {
    get(&format!("https://api.idlerpg.xyz/user?id={}", user_id))
        .call()
        .into_json_deserialize()
        .unwrap()
}

pub fn get_class_image(class: &str) -> &str {
    match class {
        "Infanterist" | "Footman" | "Shieldbearer" | "Knight" | "Warmaster" | "Templar"
        | "Paladin" => "warrior",
        "Mugger" | "Thief" | "Rogue" | "Bandit" | "Chunin" | "Renegade" | "Assassin" => "thief",
        "Juggler" | "Witcher" | "Enchanter" | "Mage" | "Warlock" | "Dark Caster"
        | "White Sorcerer" => "mage",
        "Novice" | "Proficient" | "Artisan" | "Master" | "Champion" | "Vindicator" | "Paragon" => {
            "paragon"
        }
        "Caretaker" | "Tamer" | "Trainer" | "Bowman" | "Hunter" | "Warden" | "Ranger" => "ranger",
        "Adventurer" | "Swordsman" | "Fighter" | "Swashbuckler" | "Dragonslayer" | "Raider"
        | "Eternal Hero" => "raider",
        "Priest" | "Mysticist" | "Doomsayer" | "Seer" | "Oracle" | "Prophet" | "Ritualist" => {
            "ritualist"
        }
        "No Class" => "no_class",
        _ => panic!("invalid class"),
    }
}
