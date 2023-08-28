mod tools;

use tools::Tool;

enum Defense {
    COLD,
    POISON
}

enum Size {
    SMALL,
    MEDIUM,
    LARGE
}

enum Language {
    COMMON,
    ELVISH
}

// note: I have lumped traits in with features here
struct Feature {
    name: String,
    description: String,
    reference: Optional<String>
}

struct Description {
    // TODO
}

struct HitPoints {
    hit_dice: Dice
    // TODO: in CharacterSheet impl remember to write functions for hitpoints used after 1st level and at 1st level
}

struct Class {
    name: String,
    features: Vec<Feature>,
    hit_points: HitPoints,
    proficincies: Vec<String>
}
struct Race {
    name: String,
    asi: Map<String, i64>,
    features: Vec<Feature>,
    max_age: i64,
    size: Size,
    base_walk_speed: i64,
    languages: Vec<Language>
}

pub struct CharacterSheet {
    name: String,
    age: i64,
    core_stats: Map<String, i64>,
    walking_speed: u64,
    ac: u64,
    initiative: i64,
    proficiencies: Vec<String>,
    level: i64,
    class: Class,
    race: Race,
    max_hp: i64,
    curr_hp: i64, 
    defenses: Vec<Defense>,
    saving_throws: Map<String, i64>,
    senses: Map<String, i64>,
    skills: Map<String, i64>,
    inventory: Vec<Tool>,
    features: Vec<Feature>,
    traits: Vec<Trait>,
    description: Description,
    feats: Vec<Feature>
}

impl CharacterSheet {}