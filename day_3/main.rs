use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const count: usize = 12;
    let mut array: [i32; count] = [0; count];
    let mut out_array: [i32; count] = [0; count];
    let mut out_array_inv: [i32; count] = [0; count];
    let mut line_count: i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day3_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_count += 1;
                println!("{}", ip);
                for (i, c) in ip.chars().enumerate() {
                    if c == '1' {
                        array[i] += 1;
                    }
                }
            }
        }
    }

    for (i, val) in array.iter().enumerate(){
        if val > &((line_count - val)) {
            out_array[i] = 1;
            out_array_inv[i] = 0;
        }
        else {
            out_array[i] = 0;
            out_array_inv[i] = 1;
        }
    }
    println!("{:?}", out_array);
    println!("{:?}", out_array_inv);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
