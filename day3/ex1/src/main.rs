use std::env;
use std::fs;
use std::fmt;

/*
Okay this one sucks. We're given a grid of # and .
We gotta get to the bottom by way of 3 right and 1 down
If we get to the end of a row, it repeats
Best to just make the grid wide enough at start
Then as we go down rows and over columns, if we hit a #, we count it
Find out how many trees we hit on the way to the bottom.

*/

struct Grid {
    rows: Vec<String>
}

impl Grid {
    fn new(s: String) -> Grid {
        Grid {
            rows: s.split("\n")
                .map(|r| String::from(r))
                .collect()
        }
    }
}

fn main() {

    match env::args().nth(1) {
        None    => panic!("No file provided"),
        Some(p) => {
            let grid = String::from_utf8(fs::read(p).unwrap()).unwrap();
            println!("{}", grid);
        }
    }

}
