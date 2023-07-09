mod state;
mod to_do;

use std::env;

use serde_json::json;
use state::read_file;

use crate::state::write_to_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status = &args[1];
    let title = &args[2];
    let mut state = read_file("./state.json");

    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);

    write_to_file("./state.json", &mut state);
}
