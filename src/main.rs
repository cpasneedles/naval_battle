use std::io::{self, Write};

const GRID_SIZE: usize = 10;

#[derive(Debug)]
struct Grid {
    data: [[char; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn new() -> Self {
        Grid {
            data: [['~'; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn display(&self) {
        println!("   A B C D E F G H I J");
        for (i, ligne) in self.data.iter().enumerate() {
            print!("{:2}", i + 1);
            for &case in ligne {
                print!(" {}", case);
            }
            println!();
        }
    }

    fn set_boat(&mut self, x: usize, y: usize, taille: usize, horizontal: bool) -> bool
    {
        if horizontal {
            if x + taille > GRID_SIZE {
                return false;
            }
            for i in 0..taille {
                self.data[y][x + i] = 'B';
            }
        } else {
            if y + taille > GRID_SIZE {
                return false;
            }
            for i in 0..taille {
                self.data[y + i][x] = 'B';
            }
        }
        true
    }

    fn shoot(&mut self, x: usize, y: usize) -> bool {
        match self.data[y][x] {
            'B' => {
                println!("Touché !");
                self.data[y][x] = 'X'; 
                true
            }
            '~' => {
                println!("À l'eau !");
                self.data[y][x] = 'O';                
                false
            }
            _ => {
                println!("Déjà tiré ici !");
                false
            }
        }
    }
}

fn letter_to_index(letter: char) -> Option<usize> {
    match letter {
        'A'..='J' => Some((letter as u8 - b'A') as usize),
        _ => None,
    }
}

fn read_coordinate() -> Option<(usize, usize)> {
    let mut input = String::new();
    print!("Entrez les coordonnées (ex: A5) : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    let input = input.trim().to_uppercase();
    if input.len() < 2 {
        return None;
    }

    let l = input.chars().next()?;
    let n = input[1..].parse::<usize>().ok()?;

    let x = letter_to_index(l)?;
    let y = n.checked_sub(1)?;

    if x < GRID_SIZE && y < GRID_SIZE {
        Some((x, y))
    } else {
        None
    }
}

fn main() {
    let mut grid = Grid::new();

    grid.set_boat(4, 1, 3, true);
    grid.set_boat(8, 7, 2, false);
    grid.set_boat(2, 6, 6, true);

    println!("--- Grille de Bataille Navale ---");
    grid.display();

    loop {
        if let Some((x, y)) = read_coordinate() {
            grid.shoot(x, y);
            grid.display();
        } else {
            println!("Coordonnées invalides. Essayez encore.");
        }
    }
} 
