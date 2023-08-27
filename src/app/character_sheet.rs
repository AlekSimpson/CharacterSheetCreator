mod tools;

use tools::Tool;

enum Defense {
    COLD,
    POISON
}

//enum CoreStat {
//    stren{value: i64},
//    dex{value: i64},
//    con{value: i64},
//    int{value: i64},
//    wis{value: i64},
//    rizz{value: i64}
//}

struct Feature {
    // TODO
}
struct Trait {
    // TODO
}
struct Description {
    // TODO
}
struct Class {
    // TODO
}
struct Race {
    // TODO
}

pub struct CharacterSheet {
    //core_stats: Map<String, CoreStat>,
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
    description: Description
}

impl CharacterSheet {}