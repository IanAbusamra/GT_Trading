use rand::Rng;
use std::io;

struct Point {
    x: usize, 
    y: usize,
}

fn main() {
    let mut setup_board: [i32;16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    let mut board: [[i32;4];4] = [[0;4];4];
    let mut blank_point = Point {
        x: 0,
        y: 0,
    };
    let mut game_over: bool = false;

    randomize_board(&mut setup_board);
    while !is_solvable(&mut setup_board) {
        randomize_board(&mut setup_board);
    }

    for i in 0..4 {
        for j in 0..4 {
            board[i][j] = setup_board[4 * i + j];
            if board[i][j] == 16 {
                blank_point.x = i;
                blank_point.y = j;
            }
        }
    }

    display_instructions();
    
    while !game_over {
        display_board(&mut board);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input: char = input.trim().parse().expect("Please type a character");
        let mut new_point = Point {
            x: blank_point.x,
            y: blank_point.y,
        };
        
        match input {
            'w' => new_point.x = if blank_point.x == 0 {0} else {new_point.x - 1},
            'a' => new_point.y = if blank_point.y == 0 {0} else {new_point.y - 1},
            's' => new_point.x = if blank_point.x == 3 {3} else {new_point.x + 1},
            'd' => new_point.y = if blank_point.y == 3 {3} else {new_point.y + 1},
            _ => continue,
        }
        
        let temp: i32 = board[blank_point.x][blank_point.y];
        board[blank_point.x][blank_point.y] = board[new_point.x][new_point.y];
        board[new_point.x][new_point.y] = temp;

        blank_point.x = new_point.x;
        blank_point.y = new_point.y;

        if is_solved(&mut board) {
            game_over = true;
        }
    }

    display_board(&mut board);
    println!("Congrats! You have beaten the 15 puzzle.");
}

pub fn is_solved(board: &mut [[i32; 4]; 4]) -> bool {
    let mut cur: i32 = 1;
    for i in 0..4 {
        for j in 0..4 {
            if cur != board[i][j] {
                return false;
            }
            cur += 1;
        }
    }
    return true;
}

pub fn display_instructions() {
    println!("Welcome to the 15 puzzle game!");
    println!("Please use w, a, s, d to shift up, left, down, and right, respectively.");
    println!("");
}

pub fn display_board(board: &mut [[i32; 4]; 4]) {
    for i in 0..4 {
        println!("////////////////////////");
        println!("// {} // {} // {} // {} //", decide(&board[i][0]), decide(&board[i][1]), decide(&board[i][2]), decide(&board[i][3]));
    }
    println!("////////////////////////");
}

pub fn decide(num: &i32) -> String {
    if num == &16i32 {
        return "__".to_string();
    } else {
        return num.to_string();
    }
}

pub fn randomize_board (setup_board: &mut [i32]) {
    let mut rng = rand::thread_rng();
    for i in (0..16).rev() {
        let j: usize = rng.gen_range(0..=15);
        let temp: i32 = setup_board[i];
        setup_board[i] = setup_board[j];
        setup_board[j] = temp;
    }
}

pub fn is_solvable(setup_board: &mut [i32]) -> bool {
    //naive approach for ease of implementation
    let mut cnt: i32 = 0;
    for i in 0..15 {
        for j in (i + 1)..16 {
            if setup_board[i] > setup_board[j] {
                cnt += 1;
            }
        }
    }
    return cnt % 2 == 0;
}