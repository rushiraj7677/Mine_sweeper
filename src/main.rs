use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::{io::Read, usize};

const SIDE: usize = 9;
const MINES: usize = 8;
pub fn isvalid(row: usize, col: usize) -> bool {
    return row >= 0 && row < SIDE && col >= 0 && col < SIDE;
}
fn ismine(row: usize, col: usize, v: &mut Vec<Vec<usize>>) -> bool {
    if v[row][col] == '*' as usize {
        true
    } else {
        false
    }
}
fn make_move() -> usize {
    println!("Enter Your Move ->");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    n as usize
}
fn printboard(v: &mut Vec<Vec<usize>>) {
    print!("  ");
    for i in (1..SIDE).step_by(1) {
        print!("{} ", i)
    }
    print!("\n");
    for i in (1..SIDE).step_by(1) {
        print!("{} ", i);
        for j in (1..SIDE).step_by(1) {
            let val = v
                .get(i)
                .expect("index out of bound")
                .get(j)
                .expect("index out of bound");
            let v_char: u8 = (*val).try_into().expect("value is large to fit into u8");
            print!("{} ", v_char as char);
            // print!("{} ", val);
        }
        print!("\n");
    }
    return;
}
fn count_adjacent_mines(row: usize, col: usize, realboard: &mut Vec<Vec<usize>>) -> u8 {
    let mut count: u8 = 0;
    if isvalid(row - 1, col) == true {
        if ismine(row - 1, col, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row + 1, col) == true {
        if ismine(row + 1, col, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row, col + 1) == true {
        if ismine(row, col + 1, realboard) == true {
            count = count + 1;
        }
    }
    if isvalid(row, col - 1) == true {
        if ismine(row, col - 1, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row - 1, col + 1) == true {
        if ismine(row - 1, col + 1, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row + 1, col + 1) == true {
        if ismine(row + 1, col + 1, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row + 1, col - 1) == true {
        if ismine(row + 1, col - 1, realboard) == true {
            count += 1;
        }
    }
    if isvalid(row - 1, col - 1) == true {
        if ismine(row - 1, col - 1, realboard) == true {
            count += 1;
        }
    }
    count
}

fn play_minesweeper_util(
    myboard: &mut Vec<Vec<usize>>,
    realboard: &mut Vec<Vec<usize>>,
    mines: &mut Vec<Vec<usize>>,
    row: usize,
    col: usize,
    movesleft: &mut usize,
) -> bool {
    if myboard[row][col] != '-' as usize {
        false;
    }
    if realboard[row][col] == '*' as usize {
        myboard[row][col] = '*' as usize;
        for i in (0..MINES).step_by(1) {
            myboard[mines[i][0]][mines[i][1]] = '*' as usize;
        }
        printboard(myboard);
        println!("\n You lost! \n");
        true
    } else {
        let count: u8 = count_adjacent_mines(row, col, realboard);
        *movesleft -= 1;
        let c = '0' as u8 + count;
        myboard[row][col] = c as usize;
        false
    }
}
fn placemines(mines: &mut Vec<Vec<usize>>, realboard: &mut Vec<Vec<usize>>) {
    let mut mark = vec![false; SIDE * SIDE];
    let mut count = 0;
    while count != MINES {
        let mut rng = thread_rng();
        let random = rng.gen_range(0..64) % (SIDE * SIDE);
        let x = random / SIDE;
        let y = random % SIDE;
        if mark[random] == false {
            mines[count][0] = x;
            mines[count][1] = y;
            realboard[mines[count][0]][mines[count][1]] = '*' as usize;
            mark[random] = true;
            count += 1;
        }
    }
    return;
}
fn initialise(realboard: &mut Vec<Vec<usize>>, myboard: &mut Vec<Vec<usize>>) {
    for i in (0..SIDE).step_by(1) {
        for j in (0..SIDE).step_by(1) {
            realboard[i][j] = '-' as usize;
            myboard[i][j] = realboard[i][j];
        }
    }
    return;
}
// fn replace_mines(row: usize, col: usize, board: &mut Vec<Vec<usize>>) {
//     for i in (0..SIDE).step_by(1) {
//         for j in (0..SIDE).step_by(1) {
//             if board[i][j] != '*' as usize {
//                 board[i][j] = '*' as usize;
//                 board[row][col] = '-' as usize;
//                 return;
//             }
//         }
//     }
//     return;
// }
fn play_mine_sweeper() {
    let mut gameover: bool = false;
    let mut real_board = vec![vec![48; SIDE]; SIDE];
    let mut my_board = vec![vec![48; SIDE]; SIDE];
    let mut movesleft: usize = SIDE * SIDE - MINES;
    let mut mines = vec![vec![0; 2]; MINES];
    // let x: usize;
    // let y: usize;
    initialise(&mut real_board, &mut my_board);
    placemines(&mut mines, &mut real_board);
    // let currentmoveindex = 0;
    while gameover == false {
        println!("Current Status of Board : ");
        printboard(&mut my_board);
        let x = make_move();
        let y = make_move();
        // if currentmoveindex == 0 {
        //     if ismine(x as usize, y as usize, &mut real_board) {
        //         println!("Game over!");
        //         return;
        //         // replace_mines(x, y, &mut real_board);
        //     }
        // }
        // let _ = currentmoveindex + 1;
        gameover = play_minesweeper_util(
            &mut my_board,
            &mut real_board,
            &mut mines,
            x,
            y,
            &mut movesleft,
        );
        if gameover == false && movesleft == 0 {
            println!("\n You won! ");
            gameover = true;
        }
    }
    return;
}
fn main() {
    println!("Mine Sweeper");
    play_mine_sweeper();
}
