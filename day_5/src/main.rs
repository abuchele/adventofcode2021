use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAXPOS: usize = 1000;


fn main() {
    //let mut line_vec = Vec::new();
    let mut grid = [[0; MAXPOS]; MAXPOS];
    let mut outcount = 0;
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = 0;
        }
    }

    let mut line_vec = read_file("./day5_sample.txt".to_string());

    for line in line_vec {
        let re = Regex::new(r"^([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").unwrap();
        let reg_out = re.captures_iter(&line);
        for cap in re.captures_iter(&line) {
            println!(
                "x1: {} y1: {} x2: {} y2: {} ",
                &cap[1], &cap[2], &cap[3], &cap[4]
            );
            let x1 = &cap[1].parse::<usize>().unwrap();
            let y1 = &cap[2].parse::<usize>().unwrap();
            let x2 = &cap[3].parse::<usize>().unwrap();
            let y2 = &cap[4].parse::<usize>().unwrap();
            // mark the grid
            draw_line(&mut grid, x1, y1, x2, y2);
            //pretty_print(&grid);
        }
    }
    pretty_print(&grid);

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if grid[i][j] > 1 {
                outcount += 1;
            }
        }
    }
    println!("{}", outcount);
}

pub fn draw_line<'a>(
    grid: &'a mut [[i32; MAXPOS]; MAXPOS],
    x1: &'a usize,
    y1: &'a usize,
    x2: &'a usize,
    y2: &'a usize,
) -> &'a [[i32; MAXPOS]; MAXPOS] {
    let xmin: usize = cmp::min(*x1, *x2);
    let xmax: usize = cmp::max(*x1, *x2);
    let ymin: usize = cmp::min(*y1, *y2);
    let ymax: usize = cmp::max(*y1, *y2);

    let mut xpos: usize = *x1;
    let mut ypos: usize = *y1;
    //println!("y: {} x: {}", ypos, xpos);

    if xmin == xmax {
        println!("Straight line");
        for i in ymin..ymax + 1 {
            grid[i][xmin] += 1;
        }
    } else if ymin == ymax {
        println!("Straight line");
        for i in xmin..xmax + 1 {
            grid[ymin][i] += 1;
        }
    } else if (*x1 < *x2) & (*y1 < *y2) {
        for i in xmin..xmax + 1 {
            println!("y: {} x: {}", ypos, i);
            grid[ypos][i] += 1;
            ypos += 1;
        }
    } else if (*x1 < *x2) & (*y1 > *y2) {
        for i in xmin..xmax + 1 {
            println!("y: {} x: {}", ypos, i);
            grid[ypos][i] += 1;
            if ypos > 0 {
                ypos = ypos - 1;
            }
        }
    } else if (*x1 > *x2) & (*y1 < *y2) {
        for i in (xmin..xmax + 1).rev() {
            println!("y: {} x: {}", ypos, i);
            grid[ypos][i] += 1;
            ypos += 1;
        }
    } else if (*x1 > *x2) & (*y1 > *y2) {
        for i in (xmin..xmax + 1).rev() {
            println!("y: {} x: {}", ypos, i);
            grid[ypos][i] += 1;
            if ypos > 0 {
                ypos = ypos - 1;
            }
        }
    }

    grid
}

pub fn pretty_print(grid: &[[i32; MAXPOS]; MAXPOS]) {
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }
}


fn read_file(filename: String) -> Vec<String>
{
    let mut line_vec = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_vec.push(ip);
            }
        }
    }

    line_vec
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
