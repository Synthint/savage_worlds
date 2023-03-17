

use std::cmp::min;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub parent_attribute: String,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge { //edges include hinderances
    pub name: String,
    pub description: String,
    pub rank: String, // Novice, Seasoned, Veteran, Heroic, Legendary, Major, Minor (Major and Minor are for hinderances)
    pub prerequisite: String,
    pub is_hinderance: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Power{
    pub name: String,
    pub rank: String,
    pub range: String,
    pub duration: String,
    pub effect: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Armor{
    pub name: String,
    pub armor: i32,
    pub weight: i32,
    pub cost: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon{
    pub name: String,
    pub damage: String,
    pub weight: i32,
    pub cost: i32,
    pub description: String,
    pub range: i32,
    pub armor_piercing: i32,
    pub fire_rate: i32,
    pub ammo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{
    pub name: String,
    pub cost: i32,
    pub weight: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character{
    pub name: String,
    pub race: String,
    pub experience: i32,
    pub skills: Vec<Skill>,
    pub edges: Vec<Edge>,
    pub bennies: i32,
    pub money: i32,
    pub carry_weight: i32,
    pub max_weight:i32,
    pub items: Vec<Item>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
    pub pace: i32,
    pub parry: i32,
    pub toughness: i32,
    pub charisma: i32,
    pub agility: i32,
    pub smarts: i32,
    pub spirit: i32,
    pub strength: i32,
    pub vigor: i32,
    pub notes: Vec<String>,
}

impl Character{
    fn update_carry_weight(&mut self){
        self.carry_weight = 0;
        for item in &self.items{
            self.carry_weight += item.weight;
        }
        for item in &self.weapons{
            self.carry_weight += item.weight;
        }
        for item in &self.armor{
            self.carry_weight += item.weight;
        }
    }

    fn update_parry(&mut self){
        for skill in &self.skills{
            if skill.name.eq("Fighting") {
                self.parry = (min(skill.level,4)/2) + 2;
                break;
            }
        }
    }
    
    pub fn add_skill(&mut self, new_skill: &Skill) -> bool{
        for skill in self.skills.clone() {
            if skill.name.to_lowercase().eq(&new_skill.name.to_lowercase()) {
                return false;
            }
        }
        self.skills.push(new_skill.clone());
        return true;
    }

}


