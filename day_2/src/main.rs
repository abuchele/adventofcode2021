use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut num_inc: i32 = 0;
    let mut line_vec = Vec::new();
    let mut xpos: i32 = 0;
    let mut ypos: i32 = 0;
    let mut aim: i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day2_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_vec.push(ip);
            }
        }
    }

    for line in line_vec {
        let re = Regex::new(r"^(forward|backward|up|down) ([0-9]*)$").unwrap();
        let reg_out = re.captures_iter(&line);
        
        for cap in re.captures_iter(&line) {
            println!("dir: {} amt: {} ", &cap[1], &cap[2]);
            if &cap[1] == "forward"{
                let mut change = &cap[2].parse::<i32>().unwrap();
                xpos += change;
                ypos += aim * change; 
            }
            else if &cap[1] == "backward"{
                xpos -= &cap[2].parse::<i32>().unwrap();
            }
            else if &cap[1] == "down"{
                aim += &cap[2].parse::<i32>().unwrap();
            }
            else if &cap[1] == "up"{
                aim -= &cap[2].parse::<i32>().unwrap();
            }
            else{
                println!("UNKNOWN DIRECTION {}", &cap[1]);
            }
        }
    }
    println!("xpos: {} ypos: {} aim: {}", xpos, ypos, aim);
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
