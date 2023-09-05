use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

pub fn load_json_input() -> Vec<String> {
    let file = File::open("../commons/input.json").expect("Couldn't open 'input.json' file");
    let reader = BufReader::new(file);
    let input_json: Vec<Entry> = serde_json::from_reader(reader).expect("Deserialization problem");
    return input_json.iter().map(|entry| entry.text.clone()).collect();
}

#[derive(Deserialize)]
struct Entry {
    text: String,
}

// fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
//     // Open the file in read-only mode with buffer.
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     // Read the JSON contents of the file as an instance of `User`.
//     let u = serde_json::from_reader(reader)?;

//     // Return the `User`.
//     Ok(u)
// }
