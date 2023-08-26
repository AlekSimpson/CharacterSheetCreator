use std::collections::HashMap;
use sycamore::{reactive::{Scope, create_signal}, web::Html, component, view::View};
use sycamore::view;

pub struct CoreStat {
    value: i64,
    modifier: i8,
    proficient: bool
}

impl CoreStat {
    pub fn new(value: i64, modifier: i8, proficient: bool) -> Self {
        CoreStat {
            value: value,
            modifier: modifier,
            proficient: proficient
        }
    }
}

pub struct Sheet {
    // holds character sheet data
    character_name: String,
    core_stats: HashMap<String, CoreStat>,
    derived_stats: HashMap<String, i64>,
    ac: i32
}

impl Sheet {
    pub fn new(name: String, core_stats: HashMap<String, CoreStat>, derived_stats: HashMap<String, i64>, ac: i32) -> Self {
        Sheet {
            character_name: name,
            core_stats: core_stats,
            derived_stats: derived_stats,
            ac: ac
        }
    }   
}

#[component]
pub fn SheetElement<G: Html>(cx: Scope) -> View<G> {
    // TOOD: Hook this up to get data from reading csv file
    let core_stats_loaded: HashMap<String, CoreStat> = HashMap::new();
    let derived_stats: HashMap<String, i64> = HashMap::new();
    let sheet = Sheet::new(String::from("test"), core_stats_loaded, derived_stats, 12);
    let sheet_signal = create_signal(cx, sheet);

    // render sheet info
    view! { cx,
        p {
        }
    }
}