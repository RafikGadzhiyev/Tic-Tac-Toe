use std::fmt;
use std::fmt::Display;
use std::io;

enum Player {
    X,
    O,
}

impl Clone for Player {
    fn clone(&self) -> Player {
        match self {
            Player::O => Player::O,
            Player::X => Player::X
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Player::X => write!(f, "{}", 'X'),
            Player::O => write!(f, "{}", "O"),
        }
    }
}

impl Player {
    pub fn get_player_string_representation(&self) -> char {
        match &self {
            Player::O => 'O',
            Player::X => 'X',
        }
    }
}

fn main() {
    const BOARD_CELL_COUNT: usize = 9;
    const COL_PER_ROW: usize = 3;
    const EMPTY_CELL_ATTRIBUTE: char = ' ';

    let mut board = vec![EMPTY_CELL_ATTRIBUTE; BOARD_CELL_COUNT];
    let mut is_game_ended = false;

    let mut player_input = String::new();
    let mut current_player = Player::X;

    while !is_game_ended {
        println!("==========================================");
        println!(
            "{} move. Please, choose cell",
            current_player.get_player_string_representation()
        );
        io::stdin().read_line(&mut player_input).unwrap();
        println!("==========================================");

        let parsed_player_input: usize = player_input.trim().parse().unwrap_or(0);

        let last_player = current_player.clone();

        make_move(
            &mut current_player,
            parsed_player_input,
            &mut board,
            BOARD_CELL_COUNT,
            EMPTY_CELL_ATTRIBUTE,
        );

        const TOTAL_ROWS: usize = BOARD_CELL_COUNT / COL_PER_ROW;

        for row in 1..TOTAL_ROWS + 1 {
            print!("|");

            for cell in 0..COL_PER_ROW {
                print!("{}|", board[(row - 1) * COL_PER_ROW + cell]);
            }

            println!();
        }

        player_input = String::new();

        is_game_ended = check_game_ending_state(&board,&last_player)
            || get_remain_empty_cell_count(&board, EMPTY_CELL_ATTRIBUTE) == 0
    }
}

fn toggle_player(current_player: &mut Player) {
    match current_player {
        Player::X => *current_player = Player::O,
        Player::O => *current_player = Player::X
    }
}

fn make_move(
    current_player: &mut Player,
    cell_number: usize,
    board: &mut Vec<char>,
    board_size: usize,
    empty_cell_attribute: char
) -> bool {
    if !is_possible_move(cell_number, board, board_size, empty_cell_attribute) {
        return false
    }

    board[cell_number - 1] = current_player.get_player_string_representation();
    toggle_player(current_player);

    true
}

fn is_possible_move(
    cell_number: usize,
    board: &mut Vec<char>,
    board_size: usize,
    empty_cell_attribute: char
) -> bool {
    if cell_number < 1 || cell_number > board_size {
        return false
    }

    let cell_value = board[cell_number - 1];

    if cell_value != empty_cell_attribute {
        return false
    }

    true
}

fn check_game_ending_state(board: &Vec<char>, current_player: &Player) -> bool{
    const WINNING_POSITIONS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 6],                              
        [0, 4, 8],
        [2, 4, 6]
    ];

    let current_player_string_representation = current_player.get_player_string_representation();

    for winning_position in WINNING_POSITIONS.iter() {
        let mut is_winning_position = true;
        
        for winning_cell in winning_position.iter() {
            let cell_value = board[*winning_cell];

            if cell_value != current_player_string_representation {
                is_winning_position = false;

                break;
            }
        }

        if is_winning_position {
            return true
        }
    }

    false
}

fn get_remain_empty_cell_count(
    board: &Vec<char>,
    empty_cell_attribute: char
) -> i32 {
    let mut remain_empty_cell_count = 0;

    for board_cell in board.iter() {
        if *board_cell == empty_cell_attribute {
            remain_empty_cell_count += 1;
        }
    }

    remain_empty_cell_count
}