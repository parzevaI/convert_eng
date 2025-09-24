use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashMap;

use rand::Rng;

use std::env;


fn main() {
    // get input from command line and handle bad inputs
    let mms: String = get_mms();

    let dict: HashMap<String, Vec<String>> = generate_dict("./src/mms-common-4.txt").unwrap();

    let mut mms_arr: Vec<String> = mms.chars().map(|c| c.to_string()).collect::<Vec<_>>();
    let mut rng = rand::rng(); // random number generator

    let mut output: Vec<String> = Vec::new();
    loop {
        let mut mms_word_len = rng.random_range(1..=(4.min(mms_arr.len())));
        let mut word: Option<String>;

        loop {
            // get number of len 1 to 4
            let number = mms_arr[..mms_word_len].join("");

            // convert to english word
            word = convert_mms_word_to_eng(&number, &dict);

            // check if it worked
            if word.is_some() {
                break;
            }

            // reduce number length by 1
            mms_word_len -= 1;
        }

        // push to output word array
        output.push(word.unwrap());

        // remove those digits from original array
        mms_arr.drain(0..mms_word_len);

        // break when all digits have been used
        if mms_arr.len() == 0 {
            break;
        }
    }

    println!("{}", output.join(" "));
}


fn convert_mms_word_to_eng(mms: &str, dict: &HashMap<String, Vec<String>>) -> Option<String> {
    // get list of english words
    let vec = dict.get(mms)?;

    if vec.is_empty() {
        return None
    }

    let mut rng = rand::rng(); // random number generator
    let idx = rng.random_range(0..vec.len());

    Some(vec[idx].clone())
}


fn generate_dict(path: &str) -> Option<HashMap<String, Vec<String>>> {
    // open the file in read-only mode
    let file = File::open(path).ok()?;
    let reader = io::BufReader::new(file);

    let mut dict: HashMap<String, Vec<String>> = HashMap::new();

    // iterate over lines
    for line in reader.lines() {
        let line = line.ok()?;

        let line_split: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>();

        let key = line_split[1].clone();
        let value = line_split[0].clone();

        dict.entry(key).or_insert(Vec::new()).push(value);
    }

    Some(dict)
}


// command line
fn get_mms() -> String {
    let args = env::args().collect::<Vec<String>>();
    let program = args.get(0).cloned().unwrap_or_else(|| "convert_eng".to_string());

    let usage_string = format!(
        "Usage: {program} <digits>\n\nOptions:\n  -h, --help    Show this help\n\nNotes:\n  - <digits> must contain only 0-9 with no spaces\n"
    );

    // help flag
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("{}", usage_string);
        std::process::exit(1)
    }

    // require exactly one non-flag argument
    if args.len() != 2 {
        eprintln!("Error: expected exactly one argument.\n");
        eprintln!("{}", usage_string);
        std::process::exit(1);
    }

    // must be numeric
    let mms = &args[1];
    if !mms.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Error: input must be digits (0-9) only, with no spaces.\n");
        eprintln!("{}", usage_string);
        std::process::exit(1);
    }

    mms.to_string()
}
// command line

