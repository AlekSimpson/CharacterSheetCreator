mod tools;

use tools::Tool;
use rand::Rng;
use math::round;

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

// note: I have lumped traits in with features and feats here
struct Feature {
    name: String,
    description: String,
    reference: Optional<String>
}

struct Stat {
    stat: i64,
    modifier: i64
}

impl Stat {
    pub fn new(stat: i64, modifier: i64) -> Self {
        Stat {
            stat: stat,
            modifier, modifier
        }
    }
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
    pub fn new(amount: i32, type: i32) -> Self {
        Dice {
            amount: amount,
            dice_type: type
        }
    }

    pub fn roll(sum_roll: bool) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let rolls: Vec<i32> = vec![0; amount];
        rolls.iter().map(|x| x + rng.gen(1..dice_type));
        if sum_roll {
            return rolls.iter().fold(0, |acc, x| acc + x);
        }
        return rolls;
    }
}

struct Class {
    name: String,
    features: Vec<Feature>,
    hit_dice: Dice,
    proficincies: Vec<String>
    has_darkvision: bool
    proficieny_bonuses: Vec<String>
}

struct Race {
    name: String,
    asi: Map<String, i64>,
    traits: Vec<Feature>,
    max_age: i64,
    size: Size,
    base_walk_speed: i64,
    languages: Vec<Language>,
    defenses: Vec<Language>
    proficiency_bonuses: Vec<String>
}

struct Senses {
    passive_wis: i32,
    passive_int: i32,
    darkvision: Option<i32>,  // inhereted from character race
    // TODO: possibly more things can be added here but I don't know what else rn
}

pub struct CharacterSheet {
    name: String,                     
    core_stats: Map<String, Stat>,    
    walking_speed: u64,               
    ac: u64,                          
    initiative: i64,                  
    proficiencies: Vec<String>        
    level: i32,                       
    class: Class,                     
    race: Race,                       
    max_hp: i64,                      
    curr_hp: i64,                     
    saving_throws: Map<String, i32>,  
    senses: Map<String, i32>,         
    skills: Map<String, i32>,         
    inventory: Vec<Tool>,             
    background: String,               
    characteristics: Characteristics, 
    equipped: Vec<Tool>               
}

impl CharacterSheet {
    pub fn new(
        name: String, background: String, characteristics: Characteristics, inventory: Vec<Tool>, 
        background: String, race: Race, class: Class, equipped: Vec<Tool>
    ) -> Self {
        // Generate core stats
        let core_stats = HashMap::from([
            ("STR",  roll_stat()),
            ("DEX",  roll_stat()),
            ("CON",  roll_stat()),
            ("INT",  roll_stat()),
            ("WIS",  roll_stat()),
            ("CHA",  roll_stat()),
        ]);

        let skills = HashMap::from([
            ("ACROBATICS",      core_stats.clone().get("DEX").modifier),
            ("ANIMAL_HANDLING", core_stats.clone().get("WIS").modifier),
            ("ARCANA",          core_stats.clone().get("INT").modifier),
            ("ATHLETICS",       core_stats.clone().get("STR").modifier),
            ("DECEPTION",       core_stats.clone().get("CHA").modifier),
            ("HISTORY",         core_stats.clone().get("INT").modifier),
            ("INSIGHT",         core_stats.clone().get("WIS").modifier),
            ("INTIMIDATION",    core_stats.clone().get("CHA").modifier),
            ("INVESTIGATION",   core_stats.clone().get("INT").modifier),
            ("MEDICINE",        core_stats.clone().get("WIS").modifier),
            ("NATURE",          core_stats.clone().get("INT").modifier),
            ("PERCEPTION",      core_stats.clone().get("WIS").modifier),
            ("PERFORMANCE",     core_stats.clone().get("CHA").modifier),
            ("PERSUASION",      core_stats.clone().get("CHA").modifier),
            ("RELIGION",        core_stats.clone().get("INT").modifier),
            ("SLEIGHT_OF_HAND", core_stats.clone().get("DEX").modifier),
            ("STEALTH",         core_stats.clone().get("DEX").modifier),
            ("SURVIVAL",        core_stats.clone().get("WIS").modifier),
        ]);

        let saving_throws = HashMap::from([
            ("STR", get_saving_throw(core_stats.clone().get("STR").modifier)),
            ("DEX", get_saving_throw(core_stats.clone().get("DEX").modifier)),
            ("CON", get_saving_throw(core_stats.clone().get("CON").modifier)),
            ("INT", get_saving_throw(core_stats.clone().get("INT").modifier)),
            ("WIS", get_saving_throw(core_stats.clone().get("WIS").modifier)),
            ("CHA", get_saving_throw(core_stats.clone().get("CHA").modifier)),
        ]);

        let level = 1;
        let initiative = Dice::new(1, 20).roll() + core_stats.clone().get("DEX").modifier();
        let max_hp = class.hit_dice.roll() + core_stats.clone().get("CON").modifier;
        let senses = get_senses(&class);
        let proficiency_bonus = (0.25 * level) + 1).ceil();
        let ac = get_ac();
        let speed = race.base_walk_speed + core_stats.clone().get("DEX").modifier; 
        let proficincies = get_proficiencies(&race, &class);

        // initialize sheet
        CharacterSheet {
            name: name,                     
            core_stats: core_stats,    
            walking_speed: walking_speed,               
            ac: ac,                          
            initiative: initiative,                  
            proficiencies: proficincies
            level: level,                       
            class: class,                     
            race: race,                       
            max_hp: max_hp,                      
            curr_hp: curr_hp,                     
            saving_throws: saving_throws,  
            senses: senses,         
            skills: skills,         
            inventory: inventory,             
            background: background,               
            characteristics: characteristics, 
            equipped: equipped
        }
    }

    pub fn get_proficiencies(race: &Race, class: &Class) -> Vec<String> {
        let mut race_profs = race.proficincies.clone();
        let mut class_profs = class.proficincies.clone();
        let mut profs = race_profs.extend(class_profs);
        profs.sort_unstable(); // sort list
        profs.dedup(); // remove duplicates
        return profs;
    }
 
    pub fn get_saving_throw(modifier: i32) -> i32 {
        let roll = Dice::roll(1, 20);
        return roll + modifier;
    }

    pub fn get_senses(class: &Class) -> Senses {
        let passive_wis = 10 + core_stats.clone().get("WIS").modifier;
        let passive_int = 10 + core_stats.clone().get("INT").modifier;

        Senses {
            passive_wis: passive_wis,
            passive_int: passive_int,
            darkvision: if class.has_darkvision { Some(60) } else { None }
        }
    }

    pub fn get_ac() -> u64 {
        //TODO: Implement items and armor
        let unarmored = 10;
        return unarmored + core_stats.clone().get("DEX").modifier;
    }

    pub fn roll_stat() -> Stat {
        let dice::Dice = Dice::new(4, 6); // 4d6
        let rolls::Vec<i32> = dice.roll();        

        let mut min = rolls.get(0);
        for roll in rolls.iter() {
            if roll < min {
                min = roll;
            }
        }
        let stat = rolls.iter().sum() - min;

        return Stat::new(stat, roll_modifier(stat));
    }

    pub fn roll_modifier(stat: i32) -> i32 {
        return ((stat - 10)/2).ceil();
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

    //pub fn level_up() {
    //    // increase level
    //    level += 1;
    //    // reroll hit dice
    //    roll_hit_dice();
    //    // if spell caster add new spell slots
    //    // levels 4, 8, 12, 16, 19 you can choose a feat or add two points to any score you wish
    //    // levels 5, 9, 13 and 17 your proficieny bonus increases by one
    //}
}
