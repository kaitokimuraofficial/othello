use crossterm::terminal::{Clear, ClearType};


const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
const COLOR_NONE: i32 = -1;
const COLOR_BLACK: i32 = 0;
const COLOR_WHITE: i32 = 1;

const directions: [[i32; 2]; 8] = [
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
];

const color:[&str; 2] = ["Black", "White"];

fn checkCanPut(cells: &mut [[i32;8];8], _turn:i32, _x:&usize, _y:&usize, _turnOver:bool) -> bool {
    if cells[*_y][*_x] != COLOR_NONE {
        false
    } else {
        for i in 0..8 {
            let mut x = *_x as i32;
            let mut y = *_y as i32;

            x += directions[i][0];
            y += directions[i][1];
            if(x<0) || (x>=BOARD_WIDTH as i32) || (y<0) || (y>=BOARD_HEIGHT as i32) {
                    continue;
                }
            if cells[y as usize][x as usize] == _turn {
                continue;
            }
            while true {
                x += directions[i][0];
                y += directions[i][1];

                if(x<0) || (x>=BOARD_WIDTH as i32) || (y<0) || (y>=BOARD_HEIGHT as i32) {
                    break;
                }
                if cells[y as usize][x as usize] == COLOR_NONE {
                    break;
                }
                if cells[y as usize][x as usize] == _turn {
                    if !_turnOver {
                        return true
                    }

                    let mut x2 = *_x as i32;
                    let mut y2 = *_y as i32;
                    while true {
                        cells[y2 as usize][x2 as usize] = _turn;
                        x2 += directions[i][0];
                        y2 += directions[i][1];

                        if(x2==x) && (y2==y){
                            break;
                        }
                    }
                    break;
                }
            }
        }
        return true;
        }
}

fn checkCanPutAll(cells: &[[i32;8];8], cursorX:&usize, cursorY:&usize, _turn:i32) -> bool {
    let mut copy_cells:[[i32;8];8] = *cells;
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if checkCanPut(&mut copy_cells, _turn, &cursorX, &cursorY, false) {
                return true;
            }
        }
    }
    return false;
}

fn drawBoard(cells: &[[i32;8];8], cursorX:&usize, cursorY:&usize) {
    crossterm::terminal::Clear(ClearType::All);
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if(x == *cursorX) && (y == *cursorY) {
                print!("T");
            } else {
                match &cells[y][x] {
                    -1 => print!("*"),
                    0 => print!("X"),
                    1 => print!("O"),
                    _ => unreachable!()
                }
            }
        }
        println!();
    }
}

fn main() {
    let mut cursorX:usize = 0;
    let mut cursorY:usize = 0;
    let mut turn:i32 = 0;
    let mut cells = [[COLOR_NONE; BOARD_WIDTH]; BOARD_HEIGHT];
    cells[3][3] = COLOR_WHITE;
    cells[4][4] = COLOR_WHITE;
    cells[3][4] = COLOR_BLACK;
    cells[4][3] = COLOR_BLACK;

    let mut cantPut: bool = false;

    while true {
        drawBoard(&cells, &cursorX, &cursorY);

        if cantPut {
            println!("Can't put!");
        } else {
            println!("{} turn now!\n", &color[turn as usize]);
            cantPut = false;
            let answer = String::new().trim().parse().ok().unwrap();
            match answer {
                'w' => { cursorY -= 1},
                's' => {cursorY += 1},
                'a' => {cursorX -= 1},
                'd' => {cursorX += 1},
                _   => {
                    if !checkCanPut(&mut cells, turn, &cursorX, &cursorY, false) {
                        cantPut = true;
                        break;
                    }
                    checkCanPut(&mut cells, turn, &cursorX, &cursorY, true);
                    cells[cursorY][cursorX] = turn;
                    turn ^= 1;
                    if !checkCanPutAll(&cells, &cursorX, &cursorY, turn){
                        turn ^= 1;
                    }
                    break;
                },
            }
            
        }
        if !checkCanPutAll(&cells, &cursorX, &cursorY, COLOR_WHITE) && !checkCanPutAll(&cells, &cursorX, &cursorY, COLOR_BLACK) {
            let mut count:[usize;2] = [0;2];
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    if cells[y][x]!=COLOR_NONE {
                        count[cells[y][x] as usize] += 1;
                    }
                }
            }
            drawBoard(&cells, &cursorX, &cursorY);
            if count[COLOR_BLACK as usize] == count[COLOR_WHITE as usize] {
                println!("DRAW");
            } else {
                if(count[COLOR_BLACK as usize]>=count[COLOR_WHITE as usize]) {
                    println!("BLACK WINS!");
                } else{
                    println!("WHITE WINS!");
                }
            }
        }
    }
}