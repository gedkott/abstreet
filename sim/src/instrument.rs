use abstutil;
use backtrace::Backtrace;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref BACKTRACES: Mutex<HashSet<Vec<String>>> = Mutex::new(HashSet::new());
}

pub fn capture_backtrace(event: &str) {
    let bt = Backtrace::new();
    let mut found_this_fxn = false;
    let mut calls: Vec<String> = vec![event.to_string()];
    for f in bt.frames() {
        // TODO compiler flag so capture_backtrace is usually a no-op. actually, looks like this
        // doesn't work in --release mode, so use that.
        let symbol_name = f.symbols()[0].name();
        if !symbol_name.is_some() {
            return;
        }

        let raw_name = format!("{}", symbol_name.unwrap());
        let mut raw_name_parts: Vec<&str> = raw_name.split("::").collect();
        raw_name_parts.pop();
        let name = raw_name_parts.join("::");

        if found_this_fxn {
            calls.push(name.to_string());
            if name == "sim::sim::Sim::inner_step" {
                break;
            }
        } else {
            if name.ends_with("::capture_backtrace") {
                found_this_fxn = true;
            }
        }
    }

    BACKTRACES.lock().unwrap().insert(calls);
}

pub fn save_backtraces(path: &str) {
    abstutil::write_json(path, &(*BACKTRACES.lock().unwrap())).unwrap();
}

// TODO call from all interesting methods in a few different types; maybe use macros to help
// TODO script to organize and visualize results
