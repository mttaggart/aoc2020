use std::env;
use std::fs;

/*
Okay this one sucks. We're given a grid of # and .
We gotta get to the bottom by way of 3 right and 1 down
If we get to the end of a row, it repeats
Best to just make the grid wide enough at start
Then as we go down rows and over columns, if we hit a #, we count it
Find out how many trees we hit on the way to the bottom.

*/

type Slope = (usize, usize);

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

fn descend(grid: &Grid, slope: Slope) -> usize {
    // Returns count of encountered trees
    let mut trees = 0;
    let mut pos = (0, 0);
    
    while pos.1 < grid.rows.len() {
        let row = &grid.rows[pos.1];
        // Tree check
        match row.chars().nth(pos.0) {
            Some(n) => {
                        if n == '#' {
                            trees += 1;
                        } 
                       },
            None => ()
        }

        if pos.0 + slope.0 >= row.len() {
            pos.0 = (slope.0 + pos.0) - row.len();
        } else {
            pos.0 += slope.0;
        }
        pos.1 += slope.1;

    }
    trees   
}


fn main() {

    match env::args().nth(1) {
        None    => panic!("No file provided"),
        Some(p) => {
            let slopes = vec![
                (1, 1),
                (3, 1),
                (5, 1),
                (7, 1),
                (1, 2)
            ];
            let grid = Grid::new(
                String::from_utf8(fs::read(p).unwrap()).unwrap()
            );
            let descents: usize = slopes
                .into_iter()
                .map(|s| descend(&grid, s))
                .product();
            println!("Trees encountered: {}", descents);
        }
    }

}
