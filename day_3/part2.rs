use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    mainox();
    mainco2();
}

fn mainox() {
    const count: usize = 12;
    const numlines: usize = 1000;
    let mut array: [i32; count] = [0; count];
    let mut out_array: [i32; count] = [0; count];
    let mut out_array_inv: [i32; count] = [0; count];
    let mut line_count: i32 = 0;
    let mut searchchar: char = '0';
    let mut line_vec = Vec::new();
    let mut line_vec_cpy = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day3_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_vec.push(ip);
                line_vec_cpy = copy(&line_vec);
            }
        }
        for i in 0..count {
            line_count = 0;
            for line in &line_vec {
                line_count += 1;
                //println!("{}", line);
                if line.chars().nth(i).unwrap() == '1' {
                    array[i] += 1;
                }
            }
            if array[i] >= (line_count - array[i]) {
                searchchar = '1';
            } else {
                searchchar = '0';
            }
            let mut numremoved = 0;
            for (linenum, line) in line_vec.iter().enumerate() {
                    let mut thischar = line.chars().nth(i).unwrap();
                if line.chars().nth(i).unwrap() != searchchar {
                    line_vec_cpy.remove(linenum - numremoved);
                    numremoved += 1;
                }
            }
            line_vec = copy(&line_vec_cpy);
        }
    }
            
    println!("{:?}", line_vec_cpy);
}

fn mainco2() {
    const count: usize = 12;
    const numlines: usize = 1000;
    let mut array: [i32; count] = [0; count];
    let mut out_array: [i32; count] = [0; count];
    let mut out_array_inv: [i32; count] = [0; count];
    let mut line_count: i32 = 0;
    let mut searchchar: char = '0';
    let mut line_vec = Vec::new();
    let mut line_vec_cpy = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day3_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_vec.push(ip);
                line_vec_cpy = copy(&line_vec);
            }
        }
        for i in 0..count {
            line_count = 0;
            for line in &line_vec {
                line_count += 1;
                //println!("{}", line);
                if line.chars().nth(i).unwrap() == '1' {
                    array[i] += 1;
                }
            }
            if array[i] >= (line_count - array[i]) {
                searchchar = '1';
            } else {
                searchchar = '0';
            }
            let mut numremoved = 0;
            for (linenum, line) in line_vec.iter().enumerate() {
                    let mut thischar = line.chars().nth(i).unwrap();
                if line.chars().nth(i).unwrap() == searchchar {
                    if line_vec_cpy.iter().count() > 1
                    {
                    line_vec_cpy.remove(linenum - numremoved);
                    numremoved += 1;
                    }
                }
            }
            line_vec = copy(&line_vec_cpy);
        }
    }
            
    println!("{:?}", line_vec_cpy);
}

pub fn copy<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut vec = vec.to_vec();
    vec
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
