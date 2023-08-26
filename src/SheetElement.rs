use std::collections::HashMap;

pub struct CoreStat {
    value: i64,
    modifier: i8,
    proficient: bool
}

fn build_coreStat(value: i64, modifier: i8, proficient: bool) -> CoreStat {
    CoreStat {
        value: value,
        modifier: modifier,
        proficient: proficient
    }
}

pub struct Sheet {
    // holds character sheet data
    character_name: String,
    core_stats: HashMap<String, CoreStat>,
    derived_stats: HashMap<String, i64>,
    ac: i32
}

fn build_sheet(name: String, core_stats: HashMap<String, CoreStat>, derived_stats: HashMap<String, i64>, ac: i32) -> Sheet {
    Sheet {
        character_name: name,
        core_stats: core_stats,
        derived_stats: derived_stats,
        ac: ac
    }
}

#[component]
pub fn SheetElement<G: Html>(cx: Scope) -> View<G> {
    // TOOD: Hook this up to get data from reading csv file
    let mut core_stats_loaded: HashMap<String, CoreStat> = HashMap::new();
    let mut derived_stats: HashMap<String, i64> = HashMap::new();
    let sheet = build_sheet(String::from("test"), core_stats_loaded, derived_stats, 12);
    let sheet_signal = create_signal(cx, sheet);

    view! { cx,
        // render sheet info 
        p {
            (sheet.character_name)
        }
    }
}