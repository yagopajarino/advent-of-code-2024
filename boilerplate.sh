#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <integer>"
  exit 1
fi

# Read the first argument
input_number=$1

cargo new problem_$input_number
cd problem_$input_number
mkdir input
touch input/test.txt
touch input/submit.txt

rust_file="src/main.rs"

cat <<EOL > "$rust_file"
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
}
EOL

code src/main.rs