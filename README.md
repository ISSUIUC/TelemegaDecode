to run:

Install Rust (go to https://www.rust-lang.org/tools/install and follow the instructions)
Install Node (go to https://nodejs.org/en/download and follow the instructions)

Clone this repository

1. cd to this repo (to /TeleDecode)
2. `cargo run --release -- <freqs>` (replace `<freqs>` with the frequencies we need to use, i.e 100000, 101000)
3. Open the https link in the output of cargo run (should be a 127.0.0.1)
3. open a second terminal while the first is running:
4. cd GUI
5. `npm i`
6. `node build.mjs`

Frequencies:
* Sustainer AL0: 436750000 Hz
* Sustainer AL1: 436550000 Hz
* Booster AL3:   436350000 Hz