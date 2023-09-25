mod tools;

use tools::Tool;
use rand::Rng;

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

struct Stat {
    stat: i64,
    modifier: i64
}

struct Characteristics {
    alignment: String,
    gender: String,
    eyes: String,
    size: Size,
    height: String,
    faith: String,
    hair: String,
    skin: String,
    age: String,
    weight: String
}

struct Dice {
    amount: i32,
    dice_type: i32
}

impl Dice {
    pub fn roll() -> i32 {
        let mut rng = rand::thread_rng();
        let rolls: Vec<i32> = vec![0; amount];
        rolls.iter().map(|x| x + rng.gen(1..dice_type));
        return rolls.iter().fold(0, |acc, x| acc + x);
    }
}

struct Class {
    name: String,
    features: Vec<Feature>,
    hit_dice: Dice,
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
    core_stats: Map<String, Stat>,
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
    background: String,
    characteristics: Characteristics,
    feats: Vec<Feature>
}

impl CharacterSheet {
    pub fn new(
        name: String, age: i64,
        core_stats: Map<String, Stat>, walking_speed: u64,
        ac: u64, initiative: i64,
        proficiencies: Vec<String>, level: i64,
        class: Class, race: Race,
        max_hp: i64, curr_hp: i64,
        defenses: Vec<Defense>, saving_throws: Map<String, i64>,
        senses: Map<String, i64>, skills: Map<String, i64>,
        inventory: Vec<Tool>, features: Vec<Feature>,
        traits: Vec<Trait>, background: String,
        characteristics: Characteristics, feats: Vec<Feature>
    ) -> Self {
        CharacterSheet {
            name: name, 
            age: age,
            core_stats: core_stats, 
            walking_speed: walking_speed,
            ac: ac, 
            initiative: initiative,
            proficiencies: proficiencies,
            level: level,
            class: class,
            race: race,
            max_hp: max_hp,
            curr_hp: curr_hp,
            defenses: defenses,
            saving_throws: saving_throws,
            senses: senses,
            skils: skills,
            inventory: inventory,
            features: features,
            traits: traits,
            background: background,
            characteristics: characteristics, 
            feats: feats
        }
    }

    pub fn roll_hit_dice() {
        let roll: i32 = class.hit_dice.roll();
        max_hp += roll + core_stats["CON"].modifier;
    }

    pub fn heal(d:Dice) {
        let roll = d.roll();
        if (roll + curr_hp <= max_hp) {
            curr_hp += roll + curr_hp;
        }else {
            curr_hp = max_hp;
        }
    }

    pub fn level_up() {
        // increase level
        level += 1;
        // reroll hit dice
        roll_hit_dice();
        // if spell caster add new spell slots
        // levels 4, 8, 12, 16, 19 you can choose a feat or add two points to any score you wish
        // levels 5, 9, 13 and 17 your proficieny bonus increases by one
    }
}
