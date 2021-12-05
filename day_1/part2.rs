use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut prevsum: i32 = 100000;
    let mut num_inc: i32 = 0;
    let mut prev_vec = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let my_int = ip.parse::<i32>().unwrap();
                println!("{:?}", prev_vec);
                prev_vec.push(my_int);
                if prev_vec.len() > 3 {
                    prev_vec.remove(0);
                }
                if prev_vec.len() == 3 {
                    let currsum = sum_vec(&prev_vec);
                    if currsum > prevsum {
                        num_inc += 1;
                    }
                    prevsum = currsum;
                }
            }
        }
    }

    println!("{:?}", num_inc);
}

pub fn sum_vec(vec: &[i32]) -> i32 {
    let mut ret_sum: i32 = 0;
    for num in vec {
        ret_sum = num + &ret_sum;
    }
    ret_sum
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
