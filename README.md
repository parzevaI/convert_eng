# convert_eng

Convert numbers into memorable phrases using the Mnemonic Major System (a.k.a. the Major System).

This command-line tool takes a string of digits and produces a sequence of words whose consonant sounds encode those digits. It uses a large wordlist and randomness to generate varied, memorable results.


## What is the Major System?
The Major System maps digits 0–9 to consonant sounds. Words are chosen so that, ignoring vowels and certain consonants, their consonant sounds correspond to the digits. For example, 3-1-4-1-5-9 might become a phrase like "meat turtle file pub" (your results will vary).

This project doesn’t implement phonetic parsing itself; instead it relies on a precomputed wordlist that pairs words with their Major System digit strings.


## Features
- Turns any digit string into a phrase of words encoding that number
- Randomized chunking (1–4 digits per word) for varied, natural phrases
- Random word selection per chunk from a large dictionary


## Requirements
- Rust (edition 2024). Install via https://rustup.rs if you don’t have it.


## Build and Run
From the project root:

```bash
cargo build --release
./target/release/convert_eng 3141592653589793
```

Or run in dev mode:

```bash
cargo run -- 20250924
```

Notes:
- Output is randomized on each run (no seed is currently supported), so the exact words will vary.
- The program expects only digits (0–9). Non-digit characters are not yet handled.
- The dictionary is loaded from a relative path at runtime: `./src/mms-common-4.txt`. Run from the project root so this file can be found.


## Usage
```
convert_eng <digits>
```

Examples (results will vary each run):

```bash
# Pi (first 16 digits)
./target/release/convert_eng 3141592653589793

# A date
cargo run -- 20250924

# A phone-number-like string
cargo run -- 8005551234
```


## How it works (high-level)
1. The program loads a dictionary file at `src/mms-common-4.txt`. Each line is:
   
   ```
   <word><whitespace><digits>
   ```
   
   For example:
   
   ```
   ace 0
   cake 71
   ```
   
   Internally, the dictionary is stored as `HashMap<String, Vec<String>>` keyed by the digit string (e.g., "71"), with values being the list of words that encode those digits.
2. Your input digit string is split into chunks. A random chunk length between 1 and 4 is chosen, shrinking if necessary until a matching dictionary entry is found.
3. For each chunk, a random word is selected from the corresponding word list.
4. The words are printed as a space-separated phrase.


## Known limitations / TODOs
- Input validation: currently assumes the first CLI argument exists and is digits only
- Determinism: no way to provide a seed for reproducible output
- Word quality: some entries may be overly common or undesirable for mnemonics; filtering/tuning would help
- Configurability: no flags yet (e.g., to control chunk sizes, word frequency weighting, or dictionary path)
- Robust number handling: does not parse formatted numbers (e.g., `1,234`), multiple inputs, or separators


## Custom dictionaries
You can replace or augment the bundled dictionary by editing `src/mms-common-4.txt` (or changing the hardcoded path in `src/main.rs`). The expected format is one entry per line:

```
<word><whitespace><digits>
```

All words with the same digit string will be candidates for that chunk during generation.


## Project layout
- `src/main.rs` — CLI tool and generation logic (random chunking and word selection)
- `src/mms-common-4.txt` — bundled wordlist mapping words to Major System digit strings
- `Cargo.toml` — crate metadata


## Development
- Build: `cargo build`
- Run: `cargo run -- <digits>`
- Lint/format (optional): `cargo fmt`, `cargo clippy`


## License
No license file is included. If you plan to publish or share this project, consider adding a license (e.g., MIT/Apache-2.0).
