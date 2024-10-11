use rand::{thread_rng, Rng};
use std::io::Write;
use std::{thread, time::Duration};

const CELLS: usize = 80; // x
const COLUMNS: usize = 45; // y

fn main() {
    let insert: u8 = 0;
    let mut grid = [[insert; CELLS]; COLUMNS];

    // grid = _glider(&mut grid);
    grid = _random(&mut grid);

    let mut seed = String::new();
    for y in 0..COLUMNS {
        for x in 0..COLUMNS {
            seed.push_str(grid[y][x].to_string().as_str());
        }
    }
    println!("seed with x = {CELLS} and y = {COLUMNS}: {seed}");
    advance(grid, 0);
    
}

fn advance(grid: [[u8; CELLS]; COLUMNS], generation: u32) {
    print_cells(grid, generation);
    let insert: u8 = 0;
    let mut newgrid = [[insert; CELLS]; COLUMNS];
    for y in 0..COLUMNS {
        for x in 0..CELLS {
            let mut count = 0;

            let lowx = if x == 0 { CELLS - 1 } else { x - 1 };
            let highx = if x + 1 == CELLS { 0 } else { x + 1 };
            let xarray = [lowx, x, highx];

            let lowy = if y == 0 { COLUMNS - 1 } else { y - 1 };
            let highy = if y + 1 == COLUMNS { 0 } else { y + 1 };
            let yarray = [lowy, y, highy];

            for suby in 0..3 {
                for subx in 0..3 {
                    if grid[yarray[suby]][xarray[subx]] == 1 {
                        count += 1
                    }
                }
            }
            match grid[y][x] == 1 {
                true => {
                    newgrid[y][x] = match count - 1 {
                        2 | 3 => 1,
                        _ => 0,
                    }
                }
                false => {
                    newgrid[y][x] = match count {
                        3 => 1,
                        _ => 0,
                    }
                }
            }
        }
    }
    thread::sleep(Duration::from_millis(500));
    advance(newgrid, generation +1);
}

fn print_cells(grid: [[u8; CELLS]; COLUMNS], generation: u32) {
    let mut ps: String = String::new();
    ps.push_str("\n|");
    for _ in 0..CELLS * 2 {
        ps.push('-');
    }
    ps.push_str("|\n");

    for y in 0..COLUMNS {
        ps.push('|');
        for x in 0..CELLS {
            let character = if grid[y][x] == 1 { "██" } else { "  " };
            ps.push_str(character);
        }
        ps.push_str("|\n");
    }

    ps.push('|');
    for _ in 0..CELLS * 2 {
        ps.push('-');
    }
    ps.push_str("| generation: ");
    ps.push_str(generation.to_string().as_str());

    print!("\x1B[2J"); //clear screen
    print!("{}", ps);
    std::io::stdout().flush().unwrap();
}

fn _glider(grid: &mut [[u8; CELLS]; COLUMNS]) -> [[u8; CELLS]; COLUMNS] {
    grid[0][1] = 1;
    grid[1][2] = 1;
    grid[2][0] = 1;
    grid[2][1] = 1;
    grid[2][2] = 1;
    *grid
}

fn _random(grid: &mut [[u8; CELLS]; COLUMNS]) -> [[u8; CELLS]; COLUMNS] {
    for (_x, inner) in grid.iter_mut().enumerate() {
        for (_y, element) in inner.iter_mut().enumerate() {
            let rand: u8 = thread_rng().gen_range(0..3);
            *element = rand;
        }
    }
    *grid
}
