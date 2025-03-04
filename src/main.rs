use rand::Rng;
use core::num;
use std::fmt::format;
use std::hash::Hash;
use std::{fs::read};
use std::fs::OpenOptions;
use std::io::Write;
use std::{clone, io, vec};
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod user;

lazy_static! {
    static ref COUNTER: Mutex<i32> = Mutex::new(100000);
}


fn read_file_game_history(history_of_games: &mut Vec<(Vec<(i32, i32, i32)>, Vec<(i32, i32)>)>) -> io::Result<()> {
        let mut history_of_a_game : Vec<(i32, i32, i32)> = vec![];
        let mut history_of_dice : Vec<(i32, i32)> = vec![];
        // let mut history_of_games : Vec<Vec<(i32, i32, i32)>> = vec![];
        // let mut history_of_dices : Vec<Vec<(i32, i32)>> = vec![];
        // Define the path to the file
        let path = Path::new("game_history.txt");

        // Open the file in read-only mode
        let file = File::open(&path)?;
        
        // Create a buffered reader
        let reader = io::BufReader::new(file);
        // Read the file line by line
        let mut index: i32 = 0;
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    // println!("line: {}", content);
                    if content == "0, 0, 0"{
                        
                        
                        let num = COUNTER.lock().unwrap();
                        if index >= *num {
                            break;
                        }
                        history_of_games.push((history_of_a_game.clone(), history_of_dice.clone()));
                        index += 1;
                        history_of_a_game.clear();
                    } else {
                        let tmp = content.to_string();
                        let first_num = tmp.chars().nth(0);
                        let mut first_int: i32 = 0;
                        match first_num {
                            Some(c) => {
                                // println!("{}", c);
                                first_int = c as i32 - '0' as i32;
                            },
                            None => {

                            }
                        }
                        let second_num = tmp.chars().nth(3);
                        let mut second_int: i32 = 0;
                        
                        match second_num {
                            Some(c) => {
                                if c == '-'{
                                    second_int = -2;
                                } else {
                                    let tmp = tmp.chars().nth(4);
                                    match tmp {
                                        Some(c2) => {
                                            if c2 == ',' {
                                                second_int = c as i32 - '0' as i32;

                                            } else {
                                                second_int = (c as i32 - '0' as i32) * 10 + (c2 as i32 - '0' as i32);


                                            }
                                        },
                                        None => {
                                        }
                                    }
                                } 

                            },
                            None => {
                            }
                        }
                        let mut i = (content.len() as i32) - 1;
                        while i > -1 {
                            let c = tmp.chars().nth(i as usize);
                            match c {
                                Some(c1) => {
                                    if c1 == ',' {
                                        break;
                                    }
                                }, None => {
                                }
                            }
                            i -= 1;
                        }
                        let third_num = tmp.chars().nth((i as usize) + 2);
                        let mut third_int: i32 = 0;
                        match third_num {
                            Some(c) => {
                                //println!("{}", c);
                                if c == '-'{
                                    third_int = -1;
                                } else {
                                    let tmp = tmp.chars().nth((i as usize) + 3);
                                    match tmp {
                                        Some(c2) => {
                                            
                                            third_int = (c as i32 - '0' as i32) * 10 + (c2 as i32 - '0' as i32);
                                        },
                                        None => {
                                            third_int = c as i32 - '0' as i32;
                                        }
                                    }
                                } 
                            }, None => {

                            }
                        }
                        history_of_a_game.push((first_int, second_int, third_int));
                    }
                    
                },
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
        // Define the path to the file
        let path = Path::new("dice_history.txt");
        // Open the file in read-only mode
        let file = File::open(&path)?;
        // Create a buffered reader
        let reader = io::BufReader::new(file);
        // Read the file line by line
        index = 0;
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    // println!("line: {}", content);
                    if content == "0, 0"{
                        
                        
                        let num = COUNTER.lock().unwrap();
                        if index >= *num{
                            break;
                        }
                        
                        history_of_games[index as usize].1 = history_of_dice.clone();
                        index += 1;
                        history_of_dice.clear();
                    } else {
                        let tmp = content.to_string();
                        let first_num = tmp.chars().nth(0);
                        let mut first_int: i32 = 0;
                        match first_num {
                            Some(c) => {
                                // println!("{}", c);
                                first_int = c as i32 - '0' as i32;
                                // println!("{}", first_int);
                            },
                            None => {

                            }
                        }
                        let second_num = tmp.chars().nth(3);
                        let mut second_int: i32 = 0;
                        match second_num {
                            Some(c) => {
                                // println!("{}", c);
                                second_int = c as i32 - '0' as i32;
                                // println!("{}", second_int);
                            },
                            None => {
                            }
                        }
                        history_of_dice.push((first_int, second_int));
                    }
                },
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
        history_of_games.push((history_of_a_game.clone(), history_of_dice.clone()));
        
        Ok(())
}

fn write_to_file(game_move: &(i32, i32, i32)) {
    let data = format!("{}, {}, {}\n", game_move.0, game_move.1, game_move.2);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("game_history.txt")
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
fn write_to_file_dice(dice: &(i32,i32)){
    let data = format!("{}, {}\n", dice.0, dice.1);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("dice_history.txt")
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
fn write_to_file_board(board: &Vec<(i32, i32)>, hit_stones_1: &i32, hit_stones_2: &i32, keep_count_of_1_stones: &mut i32, keep_count_of_2_stones: &mut i32){
    let mut  data = String::from("");
    for i in 0..24{
        data.push_str(&format!("({}, {}),", board[i].0 , board[i].1));
    }
    data.push_str(&format!("{}, {}, {}, {}\n", hit_stones_1, hit_stones_2, keep_count_of_1_stones, keep_count_of_2_stones));
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("board_history.txt")
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
// Display of the board
pub fn display_board(board: &Vec<(i32, i32)>, hit_stones_1: &i32, hit_stones_2: &i32, keep_count_of_1_stones: &mut i32, keep_count_of_2_stones: &mut i32) {
    const lower_nums: &str = "11 10 9  8  7  6  | 5  4  3  2  1  0";
    const upper_nums: &str = "12 13 14 15 16 17 | 18 19 20 21 22 23";
    let mut highest_lower: i32 = 0;
    let mut highest_upper: i32 = 0;
    for i in 0..12{
        if(board[i].1 > highest_lower){
            highest_lower = board[i].1;
        }
    }
    for i in 12..24{
        if(board[i].1 > highest_upper){
            highest_upper = board[i].1;
        }
    }
    println!("{upper_nums}");
    for i in 1..=highest_upper{
        let mut row_string: String = String::from("");
        for j in 12..24{
            if(board[j].1 - i >= 0){
                if(board[j].0 == 1){
                    if(j == 17){
                        row_string.push_str("*    ");
                    }else{
                        row_string.push_str("*  ");
                    }
                } else {
                    if(j == 17){
                        row_string.push_str("#    ");
                    }else{
                        row_string.push_str("#  ");
                    }
                }
            }else{
                if(j == 17){
                    row_string.push_str("     ");
                } else {
                    row_string.push_str("   ")
                }
            }
        }
        println!("{row_string}");
    }
    // println!("-------------------------------------");
    // println!("-------------------------------------"); 
    println!("");
    let mut lower_row_strings: Vec<String> = vec![];
    for i in 1..=highest_lower{
        let mut row_string: String = String::from("");
        for j in (0..12).rev(){
            if(board[j].1 - i >= 0){
                if board[j].0 == 1{
                    if j == 11 || j == 10{
                        row_string.push_str("*  ");
                    }else if j == 6 {
                        row_string.push_str("*    ")
                    }else {
                        row_string.push_str("*  ");
                    }
                } else {
                    if j == 11 || j == 10 {
                        row_string.push_str("#  ");
                    }else if j == 6{
                        row_string.push_str("#    ");
                    }else {
                        row_string.push_str("#  ");
                    }
                }
            }else{
                if j == 6{
                    row_string.push_str("     ");
                } else {
                    if j == 11 || j == 10{
                        row_string.push_str("   ");
                    } else {
                        row_string.push_str("   ")
                    }
                    
                }
            }
        }
        lower_row_strings.push(row_string);
    }
    for i in (0..lower_row_strings.len()).rev(){
        println!("{}",lower_row_strings[i]);
    }

    println!("{lower_nums}");
    println!("hit_stones_1: {}", hit_stones_1);
    println!("hit_stones_2: {}", hit_stones_2);
    println!("out_stones_1: {}", 15 - *keep_count_of_1_stones);
    println!("out_stones_2: {}", 15 - *keep_count_of_2_stones);
    if 15 - *keep_count_of_1_stones > 15 {
        println!("SSSSSS 111111 > 15");
        read_input();
    }
    if 15 - *keep_count_of_2_stones > 15 {
        println!("SSSSSS 2222222 > 15");
        read_input();
    }
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 0..=23{
        if board[i].0 == 1 && board[i].1 >= 1{
            s1 += board[i].1;
        }
        if board[i].0 == 2 && board[i].1 >= 1{
            s2 += board[i].1;
        }
    }
    println!("s1: {}, s2: {}", s1, s2);
    
}


fn update_board(board: &mut Vec<(i32, i32)>, game_move: &(i32, i32, i32), hit_stones_1: &mut i32, hit_stones_2: &mut i32, keep_count_of_1_stones: &mut i32, keep_count_of_2_stones: &mut i32){
    // going from a place to an empty place
    //write_to_file(game_move);
    if game_move.1 == -2 {
        board[game_move.2 as usize].1 -= 1;
        if board[game_move.2 as usize].1 == 0 {
            board[game_move.2 as usize].0 = 0 
        }
        if game_move.0 == 1{
            *keep_count_of_1_stones -= 1;
        } else {
            *keep_count_of_2_stones -= 1;
        }
    } else{
        if board[game_move.1 as usize].0 == 0{
            board[game_move.1 as usize].0 = game_move.0;
            board[game_move.1 as usize].1 = 1;
            if game_move.2 == -1{
                if game_move.0 == 1{
                    *hit_stones_1 -= 1;
                } else {
                    *hit_stones_2 -= 1;
                }
            } else {
                board[game_move.2 as usize].1 -= 1;
                if board[game_move.2 as usize].1 == 0{
                    board[game_move.2 as usize].0 = 0;
                }
            }
        // going from a place to a place you are and you are not
        } else{
            // your place
            if(board[game_move.1 as usize].0 == game_move.0){
                board[game_move.1 as usize].1 += 1;
                if game_move.2 == -1{
                    if game_move.0 == 1{
                        *hit_stones_1 -= 1;
                    } else {
                        *hit_stones_2 -= 1;
                    }
                } else {
                    board[game_move.2 as usize].1 -= 1;
                    if board[game_move.2 as usize].1 == 0{
                        board[game_move.2 as usize].0 = 0;
                    }
                }
            // not place
            } else {
                if board[game_move.1 as usize].0 == 1{
                    *hit_stones_1 += 1;
                } else {
                    *hit_stones_2 += 1;
                }
                if game_move.2 == -1{
                    if game_move.0 == 1 {
                        *hit_stones_1 -= 1;
                    } else {
                        *hit_stones_2 -= 1;
                    }
                }else{
                    board[game_move.2 as usize].1 -= 1;
                    if board[game_move.2 as usize].1 == 0{
                        board[game_move.2 as usize].0 = 0;
                    }
                }
                board[game_move.1 as usize].0 = game_move.0;
                board[game_move.1 as usize].1 = 1;   
            }
        }
    }
    // This is a shitty fix for whatever is
    // println!("hitstones1: {}, hitstones2: {}", *hit_stones_1, *hit_stones_2);
    // if *hit_stones_1 < 0 {
    //     *hit_stones_1 = 0;
    // }
    // if *hit_stones_2 < 0 {
    //     *hit_stones_1 = 0;
    // }
    // read_input();
}


fn read_input() -> i32 {
    loop {
        let mut input = String::new();
        
        println!("Please enter an integer:");

        // Read input from standard input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input and parse it into an i32
        match input.trim().parse::<i32>() {
            Ok(number) => return number, // Return the valid number
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

// fn handle_game(game_mode: i32, possible_moves: &Vec<(i32, i32, i32)>, board: &Vec<(i32, i32)>, dice: &(i32,i32), turn: i32, d1_used: bool, d2_used: bool, decide_dice: bool, how_many_double_used: i32) -> usize{

    
//     if game_mode == 1 {
//         return ai_v_ai(board, possible_moves, dice, turn, d1_used, d2_used, how_many_double_used);
//     } else if game_mode == 2{
//         return user_v_ai(board, possible_moves, dice, turn, d1_used, d2_used, how_many_double_used);
//     } else if game_mode == 3{
//         return user_v_user(board, possible_moves, dice, turn, d1_used, d2_used, decide_dice, how_many_double_used);
//     } else {
//         return 0;
//     }
// }

// // I need the board whose turn it is 
// // I need the dice, basically everything that there is down there
// // Board, Dice, Moves used,Turn, Out pieces, hit pieces
// // I will have this function and then will have another that goes over the graph search and caluclate everything for the stat
// // Then when i can I will  parralalize when the tree grows, and have a set to keep track if a postion is being repeated

// pub fn possible_moves_generator(board: &Vec<(i32, i32)>, dice: &(i32, i32), turn: i32, d1_used: bool, d2_used: bool, how_many_double_used, keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32){
//     let mut possible_moves: Vec<(i32, i32, i32)> = Vec::new();


//     let mut i = 0;
    
//     let mut rng = rand::thread_rng();
//     let mut win_1 = 0;
//     let mut win_2 = 0;
//     // Generate a random number in the range [0, 10)    
//     while(i < number_of_games){        
//         let mut number_of_steps: i32 = 0;
//         while keep_count_of_1_stones > 0 && keep_count_of_2_stones > 0  {
//             number_of_steps += 1;
//             let dice1: i32 = rng.gen_range(1..=6);
//             let dice2: i32 = rng.gen_range(1..=6);
//             //write_to_file_dice(&(dice1, dice2));
//             history_of_dice.push((dice1, dice2));
            
//             // Find the stones with possiblitiy of the move
//             if dice1 != dice2 {
//                 // two moves maximum to play
//                 // check whose turn it is first

//                 // START: PLAYER 11111 WITH TWO DICES NOT ALIKE
//                 if (history_of_dice.len() % 2) == 1 {
//                     // println!("PPPPPPPPP 11111111111111");
//                     // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
//                     // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     let mut d1_used: bool = false;
//                     let mut d2_used: bool = false;
//                     for i in 0..2{
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {
//                                 // START: Check hit stones for player 11111
//                                 if hit_stones_1 > 0{
//                                     // See if we have any possible moves to make and then randomly choose between them
                                    
//                                     if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
//                                         possible_moves.push((1, 24 - last_roll.0, -1));
//                                     }
//                                     if !(board[24 - (last_roll.1 as usize) ].0 == 2 && board[24 - (last_roll.1 as usize) ].1 >= 2){
//                                         possible_moves.push((1, 24 - last_roll.1, -1));
                                    
//                                     }
//                                     if possible_moves.len() as i32 >= hit_stones_1{
//                                         // choose randomly a move from the 
//                                         for i in 0..hit_stones_1{
//                                             // This rand_num will be updated with calculated algorithms
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                             history_of_a_game.push(possible_moves[rand_num].clone());
//                                             update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             if 24 - possible_moves[rand_num].1 == last_roll.0 {
//                                                 d1_used = true;
//                                             }else{
//                                                 d2_used = true;
//                                             }
//                                             possible_moves.remove(rand_num);
                                            
//                                         }
                                        
//                                         possible_moves.clear();
                                    
//                                     } else {
//                                         // Play the moves but there is still going to be hit_stones
                                        
//                                         history_of_a_game.extend(possible_moves.clone());
//                                         d1_used = true;
//                                         d2_used = true;
//                                         for i in 0..possible_moves.len(){
//                                             update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         }
//                                         possible_moves.clear();

//                                     }
//                                 // END: CHECKING THE IF PLAYER 11111 has any hit stones
//                                 // checking if any of the dices are used
//                                 } else if !d1_used || !d2_used {
//                                     // START: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE
//                                     if !d1_used && !d2_used{

//                                         // randomly deciding which dice we use 
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, true, -1);
//                                         // START: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE
//                                         if rand_num == 1{
//                                             // check if stones of player 1 can go out
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if board[i].0 == 1 {
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             // if the stone can go then we need to only check the player 11111 house
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
                                                        
//                                                         if i as i32 - (last_roll.0 ) < 0{
//                                                             // going out
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             // moving inside the house
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }

//                                             // Player 11111 can't go out, finding the right move
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD: LINE 313 {}", (last_roll.0 as usize ));
//                                                         if ((i as i32) - last_roll.0 ) >= 0 {
//                                                             if (!(board[((i as i32) - (last_roll.0 )) as usize].0 == 2 && board[((i as i32) - (last_roll.0 )) as usize].1 >= 2))  {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     //println!("1111111111");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     //println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }

//                                             d1_used = true;
//                                         // END: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE 
//                                         // START: USING THE SECOND DICE WHEN WE HAVE BOTH DICES AVAILABLE
//                                         } else {

//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 // when the player go out so all the stones should be in the house hence the 0..=5 
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         if i as i32 - (last_roll.1 ) < 0{
//                                                             // going out
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             // moving inside the hourse
//                                                                 if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
//                                                                     possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
//                                                                 }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }

//                                             // player 11111 can't go out so we have to choose a move from the board
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD LINE 378: {}", (last_roll.1 as usize));
//                                                         if ((i as i32) - last_roll.1 ) >= 0 {
//                                                             if !(board[((i as i32) - (last_roll.1 )) as usize].0 == 2 && board[((i as i32) - (last_roll.1 )) as usize].1 >= 2) {
//                                                                 possible_moves.push((1, ((i as i32) - last_roll.1 ) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                 // println!("222222222");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }
//                                     // END: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE 
//                                     // START: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
//                                     } else if (!d1_used && d2_used) || (d1_used && !d2_used){
//                                         // START: FIRST DICE IS NOT USED
//                                         if !d1_used {
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         //going out
//                                                         if (i as i32) - last_roll.0 < 0{
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0 {
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         if ((i as i32) - last_roll.0 ) >= 0 {
//                                                             // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i as i32) - (last_roll.0) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                 // println!("33333333");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num  = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true
//                                         // END: FIRST DICE IS NOT USED
//                                         }
//                                         // START: SECOND DICE IS NOT USED
//                                         if !d2_used {
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         //going out
//                                                         if (i as i32)- last_roll.1 < 0{
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num  = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD: {}", (last_roll.1 as usize));
//                                                         if ((i as i32) - last_roll.1 ) >= 0 {
//                                                             if !(board[(i as i32 - (last_roll.1 )) as usize].0 == 2 && board[(i as i32 - (last_roll.1 )) as usize].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 // println!("444444444");
//                                                 if(possible_moves.len() > 0){
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }

                                    
//                                             d2_used = true;
//                                         }
//                                         // END: SECOND DICE IS NOT USED
//                                     // END: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
//                                     } else {
//                                         // It's possible to be here 
//                                     }
//                                 }

//                             },None =>{
//                                 // There should be a roll definitely, we'll see if there is going to be anything here
//                             }
//                         }
                        
//                     }
//                 // END: PLAYER 11111 WITH TWO DICES NOT ALIKE

//                 // START: PLAYER 22222 TURN WITH DICES NOT ALIKE
//                 } else {
//                     // println!("PPPPPPPPP 2222222222222");
//                     // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     let mut d1_used: bool = false;
//                     let mut d2_used: bool = false;
//                     for i in 0..2{
                        
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {

//                                 // START: Check hit stones for player 22222
//                                 if hit_stones_2 > 0{

//                                     // See if we have any possible moves to make and then randomly choose between them
//                                     if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
//                                         // so basically the -1 is for showing the move is form a hit position
//                                         // -2 is for going out to score
//                                         possible_moves.push((2, ((last_roll.0 as usize) - 1) as i32, -1));
                                        
//                                     }
//                                     if !(board[(last_roll.1 as usize) - 1 ].0 == 1 && board[(last_roll.1 as usize) - 1 ].1 >= 2){
//                                         possible_moves.push((2,  ((last_roll.1 as usize) - 1) as i32, -1));
                                        

//                                     }
//                                     if possible_moves.len() as i32 >= hit_stones_2{
//                                         // choose randomly a move from the 
//                                         for i in 0..hit_stones_2{
//                                             // This rand_num will be updated with calculated algorithms
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                             history_of_a_game.push(possible_moves[rand_num].clone());
//                                             update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             if(possible_moves[rand_num].1 + 1 == last_roll.0){
//                                                 d1_used = true;
//                                             }else{
//                                                 d2_used = true;
//                                             }
//                                             possible_moves.remove(rand_num);
                                            
//                                         }
                                        
//                                         possible_moves.clear();

//                                     } else {
//                                         // Play the moves but there is still going to be hit_stones
                                        
//                                         history_of_a_game.extend(possible_moves.clone());
//                                         d1_used = true;
//                                         d2_used = true;
//                                         for i in 0..possible_moves.len(){
//                                             update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         }
//                                         possible_moves.clear()

//                                     }
//                                 // END: Check hit stones for player 22222
//                                 } else if !d1_used || !d2_used {
//                                     if !d1_used && !d2_used{
//                                         // randomly deciding which rand_num to use
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         let rand_num= handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, true, -1);
//                                         if(rand_num == 1){
//                                             // check if stones of player 1 can go out
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.0 as usize) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if board[i].0 == 2 {
//                                                         // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                         if (i + (last_roll.0 as usize)) <= 23{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2)  {
//                                                                 possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 111111111");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     //println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true;
//                                         } else {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.1 as usize) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[((i as i32) + last_roll.1 ) as usize].0 == 1 && board[((i as i32) + last_roll.1 )as usize].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         if ((i as i32) + (last_roll.1)) <= 23{
//                                                             if !(board[((i as i32) +(last_roll.1)) as usize].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2)  {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 2222222222");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     // println!("-------");
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }

                                        
                                        
//                                     } else if (!d1_used && d2_used) || (d1_used && !d2_used){
//                                         if !d1_used {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.0 as usize) > 23{
//                                                             possible_moves.push((2,  -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0 {
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                         if ((i as i32) + (last_roll.0)) <= 23{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 333333333333");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true
//                                         }
//                                         if !d2_used {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 1){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if (i + (last_roll.1 as usize)) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         if ((i as i32) + (last_roll.1)) <= 23{
//                                                             if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
                                                                
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }

//                                     } else {
//                                         // It's possible to be here 
//                                     }
//                                 }

//                             },None =>{
//                                 // There should be a roll definitely, we'll see if there is going to be anything here
//                             }
//                         }
                        
//                     }
//                 }
//                 // END: PLAYER 22222 TURN WITH DICES NOT ALIKE
            
//             } 
//             // This else statement is for double dices like (1,1),(2,2),(4,4)...
//             else {
//                 // println!("DOUBLE DOUBLE DOUBLE DOUBLE");
//                 if history_of_dice.len() % 2 == 1 {
//                     // println!("PPPPPPPPP 11111111111111");
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     for i in 0..4{
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {
//                                 if hit_stones_1 > 0 {
//                                     if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
                                        
//                                         let game_move = (1,  (24 - (last_roll.0 as usize)) as i32, -1);
//                                         history_of_a_game.push(game_move);
//                                         update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones)
//                                     }
//                                 } else {
//                                     let mut can_player_1_go_out = true;
//                                     for i in 6..=23{
//                                         if board[i].0 == 1{
//                                             can_player_1_go_out = false;
//                                             break;
//                                         }
//                                     }
//                                     if can_player_1_go_out {
//                                         for i in 0..=5{
//                                             if(board[i].0 == 1){
//                                                 // going out
//                                                 if((i as i32) - last_roll.0 <= -1){
//                                                     let game_move = (1, -2, i as i32);
//                                                     possible_moves.push(game_move);
                                                    
//                                                 } else {
//                                                     if !(board[i  - (last_roll.0 as usize)].0 == 2  && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, false, false, false, i + 1);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     } else {
//                                         for i in 0..=23{
//                                             if board[i].0 == 1 {
//                                                 if ((i as i32)  - (last_roll.0)) > 0 {
//                                                     if !(board[((i as i32)  - (last_roll.0)) as usize].0 == 2  && board[((i as i32)  - (last_roll.0)) as usize].1 >= 2) {
//                                                         possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, false, false, false, i + 1);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     }
//                                 }
//                             },None=>{

//                             }
//                         }
//                     }
//                 } else {
//                     //println!("PPPPPPPPP 2222222222222222");
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     for i in 0..4{
//                         match history_of_dice.last() {
//                             Some(last_roll) => {
//                                 if hit_stones_2 > 0 {
//                                     if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
                                        
//                                         let game_move = ( 2,   ((last_roll.0 as usize) - 1) as i32, -1);
//                                         history_of_a_game.push(game_move);
//                                         update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                     }
//                                 } else {
//                                     let mut can_player_2_go_out = true;
//                                     for i in 0..=17{
//                                         if board[i].0 == 2 {
//                                             can_player_2_go_out = false;
//                                             break;
//                                         }
//                                     }
//                                     if can_player_2_go_out {
//                                         for i in 18..=23{
//                                             if board[i].0 == 2 {
//                                                 // going out
//                                                 if((i as i32) + last_roll.0 > 23){
//                                                     let game_move = (2, -2, i as i32);
//                                                     possible_moves.push(game_move);
                                                    
//                                                 } else {
//                                                     if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, false, false, false, i + 1);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     } else {
//                                         for i in 0..=23{
//                                             if board[i].0 == 2 {
//                                                 // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                 if i + (last_roll.0 as usize) <= 23{
//                                                     if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
//                                                     }                                                    
//                                                 }

//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, false, false, false, i + 1);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     }
//                                 }
//                             }, None => {
//                             }
//                         }
//                     }
//                 }
//             }
//             // println!("number of steps: {}", number_of_steps);      
//             if keep_count_of_1_stones > 15 {
//                 read_input();
//             }
//             if keep_count_of_2_stones > 15 {
//                 read_input();
//             }
//             if keep_count_of_1_stones <= 0{
//                 win_1 += 1;
//             }
//             if keep_count_of_2_stones <= 0{
//                 win_2 += 1;
//             }
//         }

//         //read_input();
//         i += 1;
//         write_to_file(&(0, 0, 0));
//         write_to_file_dice(&(0, 0));
//         // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
        
//     }
//     println!("win rate: {}, {}", win_1, win_2);
// }
// make one ai better than the other one, and then cycle through that
// generate the possible moves outside of the play 
// pub fn ai_v_ai(board: &Vec<(i32, i32)>, possible_moves: &Vec<(i32, i32, i32)>, dice: &(i32,i32), turn: i32, d1_used: bool, d2_used: bool, how_many_double_used: i32) -> usize {
//     // Hyperparameters for pure 
//     return 0;
// }
// // write the graph first
// // then we decide to do what the fuck we can do
// pub fn user_v_ai(board: &Vec<(i32, i32)>, possible_moves: &Vec<(i32, i32, i32)>, dice: &(i32,i32), turn: i32, d1_used: bool, d2_used: bool, how_many_double_used: i32) -> usize{
//     // parent node index, the node is the move itself, have vector to the children, stats,  weight for those stats, updated parent, dices, d1_used, d2_used, how_many_double_used
//     // Opponent stat, opponent weight, whosemovethisis (me or the opponent)
//     // Use a replicate of play(...) to generate the children
//     // we start with accumulating the stat
//     // Have a weight vec so the importance of those stats can be determined
//     // we have the issue for decide dice that 
//     //let mut graph_search: Vec<(i32, (i32, i32, i32), Vec<i32>, Vec<i32>, Vec<f32>, i32, (i32, i32), bool, bool, i32, Vec<i32, Vec<f32>, i32)> = vec![];
//     if dice.0 != dice.1{
//         if turn == 1{
//             println!("AI 1 turn")
//             if decide_dice{
//                 // Have a loop here  so it alternates between the possiblities
//                 // The heavy math of indexing will go here probably
                

//             } else {
//                 // 

//             }
//         } else {
//             println!("User turn");
//             if decide_dice{
//                 println!("Decide which dice to use");
//                 println!("DICES: 1. {}, 2. {}", dice.0, dice.1);
//                 let decision: usize = read_input() as usize;
//                 return decision;
//             }else{
//                 println!("Decide a move:");
//                 for i in 0..possible_moves.len(){
//                     println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//                 }
//                 let decision: usize = read_input() as usize;
//                 return decision; 
//             }
//         }
//     } else {
//         if turn == 1{
//             println!("AI 1 turn");


//         } else {
//             println!("User turn");
//             println!("User player 1 decide a move:");
//             for i in 0..possible_moves.len(){
//                 println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//             }
//             let decision: usize = read_input() as usize;
//             return decision;
//         }
//     }
//     return 0;
// }

// // Are we going for three same things
// // maybe we return -1 
// pub fn user_v_user(board: &Vec<(i32, i32)>, possible_moves: &Vec<(i32, i32, i32)>, dice: &(i32,i32), turn: i32, d1_used: bool, d2_used: bool, decide_dice: bool, how_many_double_used: i32) -> usize{
//     if dice.0 != dice.1 {
//         if turn == 1{
//             println!("User Player 1 Turn");
//             if decide_dice{
//                 println!("Decide which dice to use");
//                 println!("DICES: 1. {}, 2. {}", dice.0, dice.1);
//                 let decision: usize = read_input() as usize;
//                 return decision;
//             } else {
//                 println!("Decide a move:");
//                 for i in 0..possible_moves.len() {
//                     println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//                 } 
//                 let decision: usize = read_input() as usize;
//                 return decision;
//             }

//         } else {
//             println!("User Player 2 Turn");
//             if decide_dice{
//                 println!("Decide which dice to use");
//                 println!("DICES: 1. {}, 2. {}", dice.0, dice.1);
//                 let decision: usize = read_input() as usize;
//                 return decision;

//             } else {
//                 println!("Decide a move:");
//                 for i in 0..possible_moves.len(){
//                     println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//                 }
//                 let decision: usize = read_input() as usize;
//                 return decision;
//             }
//         }
//     } else {
//         if turn == 1{
//             println!("User player 1 decide a move:");
//             for i in 0..possible_moves.len(){
//                 println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//             }
//             let decision: usize = read_input() as usize;
//             return decision;

//         } else {
//             println!("User player 2 decide a move:");
//             for i in 0..possible_moves.len(){
//                 println!("{}. ( {}, {}, {} )", i, possible_moves[i].0, possible_moves[i].1, possible_moves[i].2);
//             }
//             let decision: usize = read_input() as usize;
//             return decision;

//         }
//     }
//     return 0;
// }


/* 
    play 
    backgammon game simulation
*/
// pub fn play(number_of_games: i32, history_of_games: &mut Vec<( Vec<(i32, i32, i32)>, Vec<(i32, i32)>)>, game_mode: i32){
//     let mut i = 0;
    
//     let mut rng = rand::thread_rng();
//     let mut win_1 = 0;
//     let mut win_2 = 0;
//     // Generate a random number in the range [0, 10)    
//     while(i < number_of_games){
//         // I need to know the number of positions available,
//         // Get the legal moves and choose between them
//         let mut board: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
//                                           (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
//                                           (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
//                                           (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
        
//         let mut history_of_a_game: Vec<(i32, i32, i32)> = vec![];
//         let mut history_of_dice: Vec<(i32, i32)> = vec![];
//         let mut keep_count_of_1_stones: i32 = 15;
//         let mut keep_count_of_2_stones: i32 = 15;
//         let mut hit_stones_1: i32 = 0;
//         let mut hit_stones_2: i32 = 0;
//         let mut number_of_steps: i32 = 0;
//         while keep_count_of_1_stones > 0 && keep_count_of_2_stones > 0  {
//             number_of_steps += 1;
//             let dice1: i32 = rng.gen_range(1..=6);
//             let dice2: i32 = rng.gen_range(1..=6);
//             //write_to_file_dice(&(dice1, dice2));
//             history_of_dice.push((dice1, dice2));
//             let mut possible_moves: Vec<(i32, i32, i32)> = Vec::new();
//             // Find the stones with possiblitiy of the move
//             if dice1 != dice2 {
//                 // two moves maximum to play
//                 // check whose turn it is first

//                 // START: PLAYER 11111 WITH TWO DICES NOT ALIKE
//                 if (history_of_dice.len() % 2) == 1 {
//                     // println!("PPPPPPPPP 11111111111111");
//                     // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
//                     // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     let mut d1_used: bool = false;
//                     let mut d2_used: bool = false;
//                     for i in 0..2{
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {
//                                 // START: Check hit stones for player 11111
//                                 if hit_stones_1 > 0{
//                                     // See if we have any possible moves to make and then randomly choose between them
                                    
//                                     if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
//                                         possible_moves.push((1, 24 - last_roll.0, -1));
//                                     }
//                                     if !(board[24 - (last_roll.1 as usize) ].0 == 2 && board[24 - (last_roll.1 as usize) ].1 >= 2){
//                                         possible_moves.push((1, 24 - last_roll.1, -1));
                                    
//                                     }
//                                     if possible_moves.len() as i32 >= hit_stones_1{
//                                         // choose randomly a move from the 
//                                         for i in 0..hit_stones_1{
//                                             // This rand_num will be updated with calculated algorithms
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[rand_num].clone());
//                                             update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             if 24 - possible_moves[rand_num].1 == last_roll.0 {
//                                                 d1_used = true;
//                                             }else{
//                                                 d2_used = true;
//                                             }
//                                             possible_moves.remove(rand_num);
                                            
//                                         }
                                        
//                                         possible_moves.clear();
                                    
//                                     } else {
//                                         // Play the moves but there is still going to be hit_stones
                                        
//                                         history_of_a_game.extend(possible_moves.clone());
//                                         d1_used = true;
//                                         d2_used = true;
//                                         for i in 0..possible_moves.len(){
//                                             update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         }
//                                         possible_moves.clear();

//                                     }
//                                 // END: CHECKING THE IF PLAYER 11111 has any hit stones
//                                 // checking if any of the dices are used
//                                 } else if !d1_used || !d2_used {
//                                     // START: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE
//                                     if !d1_used && !d2_used{

//                                         // randomly deciding which dice we use 
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, true, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                         // START: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE
//                                         if rand_num == 1{
//                                             // check if stones of player 1 can go out
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if board[i].0 == 1 {
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             // if the stone can go then we need to only check the player 11111 house
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
                                                        
//                                                         if i as i32 - (last_roll.0 ) < 0{
//                                                             // going out
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             // moving inside the house
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }

//                                             // Player 11111 can't go out, finding the right move
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD: LINE 313 {}", (last_roll.0 as usize ));
//                                                         if ((i as i32) - last_roll.0 ) >= 0 {
//                                                             if (!(board[((i as i32) - (last_roll.0 )) as usize].0 == 2 && board[((i as i32) - (last_roll.0 )) as usize].1 >= 2))  {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     //println!("1111111111");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     //println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }

//                                             d1_used = true;
//                                         // END: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE 
//                                         // START: USING THE SECOND DICE WHEN WE HAVE BOTH DICES AVAILABLE
//                                         } else {

//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 // when the player go out so all the stones should be in the house hence the 0..=5 
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         if i as i32 - (last_roll.1 ) < 0{
//                                                             // going out
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             // moving inside the hourse
//                                                                 if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
//                                                                     possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
//                                                                 }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }

//                                             // player 11111 can't go out so we have to choose a move from the board
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD LINE 378: {}", (last_roll.1 as usize));
//                                                         if ((i as i32) - last_roll.1 ) >= 0 {
//                                                             if !(board[((i as i32) - (last_roll.1 )) as usize].0 == 2 && board[((i as i32) - (last_roll.1 )) as usize].1 >= 2) {
//                                                                 possible_moves.push((1, ((i as i32) - last_roll.1 ) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                 // println!("222222222");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }
//                                     // END: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE 
//                                     // START: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
//                                     } else if (!d1_used && d2_used) || (d1_used && !d2_used){
//                                         // START: FIRST DICE IS NOT USED
//                                         if !d1_used {
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         //going out
//                                                         if (i as i32) - last_roll.0 < 0{
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0 {
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         if ((i as i32) - last_roll.0 ) >= 0 {
//                                                             // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                             if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i as i32) - (last_roll.0) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                 // println!("33333333");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num  = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true
//                                         // END: FIRST DICE IS NOT USED
//                                         }
//                                         // START: SECOND DICE IS NOT USED
//                                         if !d2_used {
//                                             let mut can_player1_go_out = true;
//                                             for i in 6..=23{
//                                                 if(board[i].0 == 1){
//                                                     can_player1_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player1_go_out {
//                                                 for i in 0..=5{
//                                                     if(board[i].0 == 1){
//                                                         //going out
//                                                         if (i as i32)- last_roll.1 < 0{
//                                                             possible_moves.push((1, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num  = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 1){
//                                                         // println!("DDDDD: {}", (last_roll.1 as usize));
//                                                         if ((i as i32) - last_roll.1 ) >= 0 {
//                                                             if !(board[(i as i32 - (last_roll.1 )) as usize].0 == 2 && board[(i as i32 - (last_roll.1 )) as usize].1 >= 2) {
//                                                                 possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 // println!("444444444");
//                                                 if(possible_moves.len() > 0){
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }

                                    
//                                             d2_used = true;
//                                         }
//                                         // END: SECOND DICE IS NOT USED
//                                     // END: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
//                                     } else {
//                                         // It's possible to be here 
//                                     }
//                                 }

//                             },None =>{
//                                 // There should be a roll definitely, we'll see if there is going to be anything here
//                             }
//                         }
                        
//                     }
//                 // END: PLAYER 11111 WITH TWO DICES NOT ALIKE

//                 // START: PLAYER 22222 TURN WITH DICES NOT ALIKE
//                 } else {
//                     // println!("PPPPPPPPP 2222222222222");
//                     // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     let mut d1_used: bool = false;
//                     let mut d2_used: bool = false;
//                     for i in 0..2{
                        
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {

//                                 // START: Check hit stones for player 22222
//                                 if hit_stones_2 > 0{

//                                     // See if we have any possible moves to make and then randomly choose between them
//                                     if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
//                                         // so basically the -1 is for showing the move is form a hit position
//                                         // -2 is for going out to score
//                                         possible_moves.push((2, ((last_roll.0 as usize) - 1) as i32, -1));
                                        
//                                     }
//                                     if !(board[(last_roll.1 as usize) - 1 ].0 == 1 && board[(last_roll.1 as usize) - 1 ].1 >= 2){
//                                         possible_moves.push((2,  ((last_roll.1 as usize) - 1) as i32, -1));
                                        

//                                     }
//                                     if possible_moves.len() as i32 >= hit_stones_2{
//                                         // choose randomly a move from the 
//                                         for i in 0..hit_stones_2{
//                                             // This rand_num will be updated with calculated algorithms
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let rand_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[rand_num].clone());
//                                             update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             if(possible_moves[rand_num].1 + 1 == last_roll.0){
//                                                 d1_used = true;
//                                             }else{
//                                                 d2_used = true;
//                                             }
//                                             possible_moves.remove(rand_num);
                                            
//                                         }
                                        
//                                         possible_moves.clear();

//                                     } else {
//                                         // Play the moves but there is still going to be hit_stones
                                        
//                                         history_of_a_game.extend(possible_moves.clone());
//                                         d1_used = true;
//                                         d2_used = true;
//                                         for i in 0..possible_moves.len(){
//                                             update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         }
//                                         possible_moves.clear()

//                                     }
//                                 // END: Check hit stones for player 22222
//                                 } else if !d1_used || !d2_used {
//                                     if !d1_used && !d2_used{
//                                         // randomly deciding which rand_num to use
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         let rand_num= handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, true, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                         if(rand_num == 1){
//                                             // check if stones of player 1 can go out
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.0 as usize) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if board[i].0 == 2 {
//                                                         // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                         if (i + (last_roll.0 as usize)) <= 23{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2)  {
//                                                                 possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 111111111");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     //println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true;
//                                         } else {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.1 as usize) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[((i as i32) + last_roll.1 ) as usize].0 == 1 && board[((i as i32) + last_roll.1 )as usize].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         if ((i as i32) + (last_roll.1)) <= 23{
//                                                             if !(board[((i as i32) +(last_roll.1)) as usize].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2)  {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 2222222222");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     // println!("-------");
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }

                                        
                                        
//                                     } else if (!d1_used && d2_used) || (d1_used && !d2_used){
//                                         if !d1_used {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 2){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if i + (last_roll.0 as usize) > 23{
//                                                             possible_moves.push((2,  -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0 {
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                         if ((i as i32) + (last_roll.0)) <= 23{
//                                                             if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     // println!("2: 333333333333");
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     // println!("-------");
//                                                     // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                     // println!("WE SHOULD BE HERE");
//                                                     // println!("{}, {}", last_roll.0, last_roll.1);
//                                                     // read_input();
//                                                 }
//                                             }
//                                             d1_used = true
//                                         }
//                                         if !d2_used {
//                                             let mut can_player2_go_out = true;
//                                             for i in 0..=17{
//                                                 if(board[i].0 == 1){
//                                                     can_player2_go_out = false;
//                                                     break;
//                                                 }
//                                             }
//                                             if can_player2_go_out {
//                                                 for i in 18..=23{
//                                                     if(board[i].0 == 2){
//                                                         //going out
//                                                         if (i + (last_roll.1 as usize)) > 23{
//                                                             possible_moves.push((2, -2, i as i32));
//                                                         }else{
//                                                             if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
                                                        
//                                                     }
//                                                 }
//                                                 // going to choose a random possible move and play it
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             } else{
//                                                 for i in 0..=23{
//                                                     if(board[i].0 == 2){
//                                                         if ((i as i32) + (last_roll.1)) <= 23{
//                                                             if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
                                                                
//                                                                 possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
//                                                             }
//                                                         }
//                                                     }
//                                                 }
//                                                 if possible_moves.len() > 0{
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     let ran_num =  handle_game(game_mode, &possible_moves, &board, last_roll, 2, d1_used, d2_used, false, -1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                                     history_of_a_game.push(possible_moves[ran_num].clone());
//                                                     update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                                     possible_moves.clear();
//                                                 }
//                                             }
//                                             d2_used = true;
//                                         }

//                                     } else {
//                                         // It's possible to be here 
//                                     }
//                                 }

//                             },None =>{
//                                 // There should be a roll definitely, we'll see if there is going to be anything here
//                             }
//                         }
                        
//                     }
//                 }
//                 // END: PLAYER 22222 TURN WITH DICES NOT ALIKE
            
//             } 
//             // This else statement is for double dices like (1,1),(2,2),(4,4)...
//             else {
//                 // println!("DOUBLE DOUBLE DOUBLE DOUBLE");
//                 if history_of_dice.len() % 2 == 1 {
//                     // println!("PPPPPPPPP 11111111111111");
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     for i in 0..4{
//                         match history_of_dice.last() {
//                             Some(last_roll ) => {
//                                 if hit_stones_1 > 0 {
//                                     if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
                                        
//                                         let game_move = (1,  (24 - (last_roll.0 as usize)) as i32, -1);
//                                         history_of_a_game.push(game_move);
//                                         update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones)
//                                     }
//                                 } else {
//                                     let mut can_player_1_go_out = true;
//                                     for i in 6..=23{
//                                         if board[i].0 == 1{
//                                             can_player_1_go_out = false;
//                                             break;
//                                         }
//                                     }
//                                     if can_player_1_go_out {
//                                         for i in 0..=5{
//                                             if(board[i].0 == 1){
//                                                 // going out
//                                                 if((i as i32) - last_roll.0 <= -1){
//                                                     let game_move = (1, -2, i as i32);
//                                                     possible_moves.push(game_move);
                                                    
//                                                 } else {
//                                                     if !(board[i  - (last_roll.0 as usize)].0 == 2  && board[i - (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, false, false, false, i + 1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     } else {
//                                         for i in 0..=23{
//                                             if board[i].0 == 1 {
//                                                 if ((i as i32)  - (last_roll.0)) > 0 {
//                                                     if !(board[((i as i32)  - (last_roll.0)) as usize].0 == 2  && board[((i as i32)  - (last_roll.0)) as usize].1 >= 2) {
//                                                         possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 1, false, false, false, i + 1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     }
//                                 }
//                             },None=>{

//                             }
//                         }
//                     }
//                 } else {
//                     //println!("PPPPPPPPP 2222222222222222");
//                     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                     for i in 0..4{
//                         match history_of_dice.last() {
//                             Some(last_roll) => {
//                                 if hit_stones_2 > 0 {
//                                     if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
                                        
//                                         let game_move = ( 2,   ((last_roll.0 as usize) - 1) as i32, -1);
//                                         history_of_a_game.push(game_move);
//                                         update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                         display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                     }
//                                 } else {
//                                     let mut can_player_2_go_out = true;
//                                     for i in 0..=17{
//                                         if board[i].0 == 2 {
//                                             can_player_2_go_out = false;
//                                             break;
//                                         }
//                                     }
//                                     if can_player_2_go_out {
//                                         for i in 18..=23{
//                                             if board[i].0 == 2 {
//                                                 // going out
//                                                 if((i as i32) + last_roll.0 > 23){
//                                                     let game_move = (2, -2, i as i32);
//                                                     possible_moves.push(game_move);
                                                    
//                                                 } else {
//                                                     if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, false, false, false, i + 1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     } else {
//                                         for i in 0..=23{
//                                             if board[i].0 == 2 {
//                                                 // println!("DDDDD: {}", (last_roll.0 as usize));
//                                                 if i + (last_roll.0 as usize) <= 23{
//                                                     if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
//                                                         possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
//                                                     }                                                    
//                                                 }

//                                             }
//                                         }
//                                         if possible_moves.len() > 0{
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             let ran_num = handle_game(game_mode, &possible_moves, &board, last_roll, 2, false, false, false, i + 1,  keep_count_of_1_stones: i32,  keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32);
//                                             history_of_a_game.push(possible_moves[ran_num].clone());
//                                             update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
//                                             possible_moves.clear();
//                                         }
//                                     }
//                                 }
//                             }, None => {
//                             }
//                         }
//                     }
//                 }
//             }
//             // println!("number of steps: {}", number_of_steps);      
//             if keep_count_of_1_stones > 15 {
//                 read_input();
//             }
//             if keep_count_of_2_stones > 15 {
//                 read_input();
//             }
//             if keep_count_of_1_stones <= 0{
//                 win_1 += 1;
//             }
//             if keep_count_of_2_stones <= 0{
//                 win_2 += 1;
//             }
//         }

//         //read_input();
//         i += 1;
//         write_to_file(&(0, 0, 0));
//         write_to_file_dice(&(0, 0));
//         // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
        
//     }
//     println!("win rate: {}, {}", win_1, win_2);
// }

// pub fn mut_vec(v: &mut Vec<i32>){
//     let mut i: i32 = 0;
//     while(i < 32){
//         v.push(i);
//         i += 1;
//     }
// }

// stats -> Vec<i32>: 
// the distance of all stones to home
// How many stones are stacked
// How many stones are being hit 
// How many stones are hit by the player
// 
fn calc_stats_board(board: &Vec<(i32, i32)>, keep_count_of_1_stones: i32, keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32){
    // calculate the distance to zero for both sides
    let possible_boards: Vec<i32> = vec![];
    let mut distance_1: i32 = 0;
    let mut distance_2: i32 = 0;
    let mut singles_1: i32 = 0;
    let mut singles_2: i32 = 0;
    // distance from -1 for 1
    // distance from 24 from 2
    // need to add different things down here
    for i in 0..=23{
        if board[i].0 == 1 {
            distance_1 = distance_1 + (board[i].1 * (i as i32 + 1));
            if board[i].1 == 1{
                singles_1 = singles_1 + 1;
            }
        }

        if board[i].0 == 2 {
            distance_2 = distance_2 + (board[i].1 * (24 - i as i32));
            if board[i].1 == 1{
                singles_2 = singles_2 + 1;
            }
        }
    }
    
    println!("distance_1: {}", distance_1);
    println!("distance_2: {}", distance_2);
    println!("singles_1: {}", singles_1);
    println!("singles_2: {}", singles_2);
}

// fn generate_possible_boards_dice(board: &Vec<(i32, i32)>, dice: (i32, i32), turn: i32){
//     let possible_boards: Vec<(Vec<(i32, i32)>, Vec<i32>)> = vec![];
//     for i in 0..=24{

//     }

// }

#[derive(Hash, Eq, PartialEq, Clone)]
struct board_state{
    board: Vec<(i32, i32)>,
    keep_count_1: i32,
    keep_count_2: i32,
    hit_stones_1: i32,
    hit_stones_2: i32,
    dice: (i32, i32),
    parent_index: i32,
}
use std::collections::HashSet;
// the order does matter, but the kinds that make the same board kinda don't matter?
fn generate_boards(board: &Vec<(i32, i32)>, turn: i32, keep_count_of_1_stones:i32 , keep_count_of_2_stones: i32, hit_stones_1: i32, hit_stones_2: i32){
    let possible_boards: Vec<(Vec<(i32, i32)>, Vec<i32>)> = vec![];
    let possible_dices: Vec<(i32, i32)> = vec![(1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (2,1), (2,2), (2,3), (2,4), (2,5), (2,6), (3,1), (3,2), (3,3), (3,4), (3,5), (3,6), (4,1), (4,2), (4,3), (4,4), (4,5), (4,6), (5,1), (5,2), (5,3), (5,4), (5,5), (5,6), (6,1), (6,2), (6,3), (6,4), (6,5), (6,6)];
    // If we find the same exists we add the probablity?

    let mut all_set_board: HashSet<board_state> = HashSet::new();
    for dice in possible_dices{
        read_input();
        println!("Generate_boards dice: {}, {}", dice.0, dice.1);
        if dice.0 != dice.1{
            //
            if turn == 1{
                if hit_stones_1 > 0{
                    // For hit stones we need to settle hit_stones first
                    // finding the spots we can land on
                    // we need to know how many 
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    let mut hit_stones_1_clone = hit_stones_1;
                    let mut hit_stones_2_clone = hit_stones_2;
                    let mut board_clone = board.clone();

                    if hit_stones_1 == 1{
                        if !(board_clone[(24 - dice.0) as usize].0 == 2 && board_clone[(24 - dice.0) as usize].1 >= 2){
                            let mut board_clone_1 = board_clone.clone();
                            let mut hit_stones_1_clone_1 = hit_stones_1;
                            let mut hit_stones_2_clone_1 = hit_stones_2;
                            hit_stones_1_clone_1 = hit_stones_1_clone_1 - 1;
                            if (board_clone[(24 - dice.0) as usize].0 == 2 && board_clone[(24 - dice.0) as usize].1 == 1){
                                hit_stones_2_clone_1 = hit_stones_2_clone_1 + 1;

                            }
                            board_clone_1[(24 - dice.0) as usize].0 = 1;
                            board_clone_1[(24 - dice.0) as usize].1 = 1;
                            let mut temporary_v: Vec<i32> = vec![];
                            for i in 0..=23{
                                if board_clone_1[23 - i].0 == 1 {
                                    temporary_v.push((23 - i) as i32);
                                }
                            }    

                            for i in temporary_v{
                                let mut board_clone_r = board_clone_1.clone();
                                let mut hit_stones_2_clone_r = hit_stones_2_clone_1;    
                                if !(board_clone_r[(i - dice.1) as usize].0 == 2 && board_clone_r[(i - dice.1) as usize].1 >= 2){
                                    if(board_clone_r[(i- dice.1) as usize].0 == 2 && board_clone_r[(i - dice.1) as usize].1 == 1){
                                        hit_stones_2_clone_r = hit_stones_2_clone_r + 1;
                                        board_clone_r[(i - dice.1) as usize].0 = 0;
                                        board_clone_r[(i - dice.1) as usize].1 = 0;
                                    }

                                    board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                    if(board_clone_r[i as usize].1 == 0){
                                        board_clone_r[i as usize].0 = 0;
                                    }
                                    board_clone_r[(i - dice.1) as usize].0 = 1;
                                    board_clone_r[(i - dice.1) as usize].1 = board_clone_r[(i - dice.1) as usize].1 + 1;
                                    set_board.insert(
                                        board_state { board: board_clone_r.clone(), 
                                            keep_count_1: keep_count_of_1_stones, 
                                            keep_count_2: keep_count_of_2_stones,
                                             hit_stones_1: hit_stones_1_clone_1, 
                                             hit_stones_2: hit_stones_2_clone_r,
                                            dice: dice.clone(),
                                            parent_index: -1 
                                        }
                                    );
                                }
                            }                     
                        }
                        if !(board_clone[(24 - dice.1) as usize].0 == 2 && board_clone[(24 - dice.1) as usize].1 >= 2){
                            let mut board_clone_1 = board_clone.clone();
                            let mut hit_stones_1_clone_1 = hit_stones_1;
                            let mut hit_stones_2_clone_1 = hit_stones_2;
                            hit_stones_1_clone_1 = hit_stones_1_clone_1 - 1;
                            if (board_clone[(24 - dice.1) as usize].0 == 2 && board_clone[(24 - dice.1) as usize].1 == 1){
                                hit_stones_2_clone_1 = hit_stones_2_clone_1 + 1;

                            }
                            board_clone_1[(24 - dice.1) as usize].0 = 1;
                            board_clone_1[(24 - dice.1) as usize].1 = 1;
                            let mut temporary_v: Vec<i32> = vec![];
                            for i in 0..=23{
                                if board[23 - i].0 == 1 {
                                    temporary_v.push((23 - i) as i32);
                                }
                            }
                            for i in temporary_v{
                                let mut board_clone_r = board_clone_1.clone();
                                let mut hit_stones_2_clone_r = hit_stones_2_clone_1;    
                                if !(board_clone_r[(i - dice.0) as usize].0 == 2 && board_clone_r[(i - dice.0) as usize].1 >= 2){
                                    if(board_clone_r[(i- dice.0) as usize].0 == 2 && board_clone_r[(i - dice.0) as usize].1 == 1){
                                        hit_stones_2_clone_r = hit_stones_2_clone_r + 1;
                                        board_clone_r[(i - dice.0) as usize].0 = 0;
                                        board_clone_r[(i - dice.0) as usize].1 = 0;
                                    }

                                    board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                    if(board_clone_r[i as usize].1 == 0){
                                        board_clone_r[i as usize].0 = 0;
                                    }

                                    board_clone_r[(i - dice.0) as usize].0 = 1;
                                    board_clone_r[(i - dice.0) as usize].1 = board_clone_r[(i - dice.0) as usize].1 + 1;
                                    set_board.insert(
                                        board_state { board: board_clone_r.clone(), 
                                            keep_count_1: keep_count_of_1_stones, 
                                            keep_count_2: keep_count_of_2_stones,
                                             hit_stones_1: hit_stones_1_clone_1, 
                                             hit_stones_2: hit_stones_2_clone_r,
                                            dice: dice.clone(),
                                            parent_index: -1
                                        }
                                    );
                                }
                            } 
                        }

                    }else{
                        if !(board_clone[(24 - dice.0) as usize].0 == 2 && board_clone[(24 - dice.0) as usize].1 >= 2){
                            hit_stones_1_clone = hit_stones_1_clone - 1;
                            if (board_clone[(24 - dice.0) as usize].0 == 2 && board_clone[(24 - dice.0) as usize].1 == 1){
                                hit_stones_2_clone = hit_stones_2_clone + 1;
                                board_clone[(24 - dice.0) as usize].0 = 0;
                                board_clone[(24 - dice.0) as usize].1 = 0;

                            }
                            board_clone[(24 - dice.0) as usize].0 = 1;
                            board_clone[(24 - dice.0) as usize].1 = board_clone[(24 - dice.0) as usize].1 + 1;

                        }
                        if !(board[(24 - dice.1) as usize].0 == 2 && board[(24 - dice.1) as usize].1 >= 2){
                            hit_stones_1_clone = hit_stones_1_clone - 1;
                            if (board_clone[(24 - dice.1) as usize].0 == 2 && board_clone[(24 - dice.1) as usize].1 == 1){
                                hit_stones_2_clone = hit_stones_2_clone + 1;
                                board_clone[(24 - dice.1) as usize].0 = 0;
                                board_clone[(24 - dice.1) as usize].1 = 0;
                            }
                            board_clone[(24 - dice.1) as usize].0 = 1;
                            board_clone[(24 - dice.1) as usize].1 = board_clone[(24 - dice.1) as usize].1 + 1;

                        }
                        set_board.insert(
                            board_state { board: board_clone.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                 hit_stones_1: hit_stones_1_clone, 
                                 hit_stones_2: hit_stones_2_clone,
                                dice: dice.clone(),
                                parent_index: -1
                            }
                        );
                        
                    }

                    // for i in temporary_v{
                    // }
                    // Check if there i
                    println!("Size hashset: {}", set_board.len());
                    for mut s in set_board{
                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                    }
                }else{
                    // Indices from the board
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    let mut temporary_v: Vec<i32> = vec![];
                    for i in 0..=23{
                        if board[23 - i].0 == 1 {
                            temporary_v.push((23 - i) as i32);
                        }
                    }
                    println!("{:?}", temporary_v);
                    // first usign the first dice
                    
                    // What is happening with the board_clone
                    // basically we are going to have a bunch of these clones pushed right?
                    // we need to calculate the rest incuding itself
                    // I think we are good to go into the next phase
                    // which is changing the rest of the board with the second dice
                    // we also need to flip the order 

                    for i in temporary_v{
                        let mut board_clone = board.clone();
                        let mut hit_stones_2_clone = hit_stones_2;    
                        let mut keep_count_1_clone = keep_count_of_1_stones;
                        // Going out
                        // we need to check if everything is in zone of going out
                        let mut all_in_zone = 1;
                        for k in 6..=23{
                            if(board_clone[k].0 == 1){
                                all_in_zone = 0;
                            }
                        }
                        if i - dice.0 < 0 && all_in_zone == 1{
                            keep_count_1_clone = keep_count_1_clone - 1;
                            board_clone[i as usize].1 = board_clone[i as usize].1 - 1;
                            if board_clone[i as usize].1 == 0 {
                                board_clone[i as usize].0 = 0
                            }
                            
                        // Moving on the board
                        } else{
                            if i - dice.0 >= 0{
                                if !(board_clone[(i - dice.0) as usize].0 == 2 && board_clone[(i - dice.0) as usize].1 > 1){
                                    if(board_clone[(i- dice.0) as usize].0 == 2 && board_clone[(i - dice.0) as usize].1 == 1){
                                        hit_stones_2_clone = hit_stones_2_clone + 1;
                                        board_clone[(i - dice.0) as usize].0 = 0;
                                        board_clone[(i - dice.0) as usize].1 = 0;
                                    }

                                    board_clone[i as usize].1 = board_clone[i as usize].1 - 1;
                                    if(board_clone[i as usize].1 == 0){
                                        board_clone[i as usize].0 = 0;
                                    }
                                    board_clone[(i - dice.0) as usize].0 = 1;
                                    board_clone[(i - dice.0) as usize].1 = board_clone[(i - dice.0) as usize].1 + 1;


                                    let mut remaining: Vec<i32> = vec![];
                                    for j in 0..=i{
                                        if (board_clone[j as usize].0 == 1){
                                            remaining.push(j);
                                        }
                                    }

                                    for r in remaining{
                                        let mut board_clone_r = board_clone.clone();
                                        let mut hit_stones_2_clone_r = hit_stones_2_clone;    
                                        let mut keep_count_1_clone_r = keep_count_1_clone;

                                        let mut all_in_zone_r = 1;

                                        for k in 6..=23{
                                            if(board_clone_r[k].0 == 1){
                                                all_in_zone_r = 0;
                                            }
                                        }

                                        if r - dice.1 < 0 && all_in_zone_r == 1{
                                            keep_count_1_clone_r = keep_count_1_clone_r - 1;
                                            board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                            if board_clone_r[i as usize].1 == 0 {
                                                board_clone_r[i as usize].0 = 0
                                            }
                                            set_board.insert(
                                                board_state{
                                                    board: board_clone_r.clone(),
                                                    keep_count_1: keep_count_1_clone_r,
                                                    keep_count_2: keep_count_of_2_stones,
                                                    hit_stones_1: hit_stones_1,
                                                    hit_stones_2: hit_stones_2_clone_r,
                                                    dice: dice.clone(), 
                                                    parent_index: -1
                                                }
                                            );
                                        // Moving on the board
                                        } else{
                                            if r - dice.1 >= 0 {
                                                if !(board_clone[(r - dice.1) as usize].0 == 2 && board_clone[(r - dice.1) as usize].1 > 1){
                                                    if(board_clone[(r - dice.1) as usize].0 == 2 && board_clone[(r - dice.1) as usize].1 == 1){
                                                        hit_stones_2_clone_r = hit_stones_2_clone_r + 1;
                                                        board_clone[(r - dice.1) as usize].0 = 0;
                                                        board_clone[(r - dice.1) as usize].1 = 0;
                                                    }
                                                    board_clone_r[r as usize].1 = board_clone_r[r as usize].1 - 1;
                                                    if(board_clone_r[r as usize].1 == 0){
                                                        board_clone_r[r as usize].0 = 0;
                                                    }
                                                    board_clone_r[(r - dice.1) as usize].0 = 1;
                                                    board_clone_r[(r - dice.1) as usize].1 = board_clone_r[(r - dice.1) as usize].1 + 1;
                                                    set_board.insert(
                                                        board_state{
                                                            board: board_clone_r.clone(),
                                                            keep_count_1: keep_count_1_clone_r,
                                                            keep_count_2: keep_count_of_2_stones,
                                                            hit_stones_1: hit_stones_1,
                                                            hit_stones_2: hit_stones_2_clone_r,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                }
                                            }
                                        }
                                    } 
                                }
                            }
                        }
                    }

                }
            
            // Turn of 2
            } else {
                // There needs to be quite bit of work going on here
                // Change the way things move
                if hit_stones_2 > 0{
                    // For hit stones we need to settle hit_stones first
                    // finding the spots we can land on
                    // we need to know how many 
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    let mut hit_stones_1_clone = hit_stones_1;
                    let mut hit_stones_2_clone = hit_stones_2;
                    let mut board_clone = board.clone();

                    // only 1 hit_stone
                    if hit_stones_2 == 1{
                        if !(board_clone[(dice.0 -1) as usize].0 == 1 && board_clone[(dice.0 -1) as usize].1 >= 1){

                            let mut board_clone_1 = board_clone.clone();
                            let mut hit_stones_1_clone_1 = hit_stones_1;
                            let mut hit_stones_2_clone_1 = hit_stones_2;
                            hit_stones_2_clone_1 = hit_stones_2_clone_1 - 1;
                            if (board_clone[(dice.0 -1) as usize].0 == 1 && board_clone[(dice.0 -1) as usize].1 == 1){
                                hit_stones_1_clone_1 = hit_stones_1_clone_1 + 1;
                            }
                            board_clone_1[(dice.0 -1) as usize].0 = 2;
                            board_clone_1[(dice.0 -1) as usize].1 = 1;
                            let mut temporary_v: Vec<i32> = vec![];
                            for i in 0..=23{
                                if board_clone_1[i].0 == 2 {
                                    temporary_v.push((i) as i32);
                                }
                            }    

                            for i in temporary_v{
                                // This needs to be updated such that, never mind, we never go out of the board that's why this doesn't need to check
                                if (i + dice.1) <= 23{
                                    let mut board_clone_r = board_clone_1.clone();
                                    let mut hit_stones_1_clone_r = hit_stones_1_clone_1;    
                                    if !(board_clone_r[(i + dice.1) as usize].0 == 1 && board_clone_r[(i + dice.1) as usize].1 >= 2){
                                        if(board_clone_r[(i + dice.1) as usize].0 == 1 && board_clone_r[(i + dice.1) as usize].1 == 1){
                                            hit_stones_1_clone_r = hit_stones_1_clone_r + 1;
                                            board_clone_r[(i + dice.1) as usize].0 = 0;
                                            board_clone_r[(i + dice.1) as usize].1 = 0;
                                        }
    
                                        board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                        if(board_clone_r[i as usize].1 == 0){
                                            board_clone_r[i as usize].0 = 0;
                                        }
                                        board_clone_r[(i + dice.1) as usize].0 = 2;
                                        board_clone_r[(i + dice.1) as usize].1 = board_clone_r[(i + dice.1) as usize].1 + 1;
                                        set_board.insert(
                                            board_state { board: board_clone_r.clone(), 
                                                keep_count_1: keep_count_of_1_stones, 
                                                keep_count_2: keep_count_of_2_stones,
                                                 hit_stones_1: hit_stones_1_clone_r, 
                                                 hit_stones_2: hit_stones_2_clone_1,
                                                dice: dice.clone(),
                                                parent_index: -1
                                            }
                                        );
                                    }
                                }
                            }                     
                        }
                        if !(board_clone[( dice.1 - 1 ) as usize].0 == 1 && board_clone[(dice.1 - 1) as usize].1 >= 2){
                            let mut board_clone_1 = board_clone.clone();
                            let mut hit_stones_1_clone_1 = hit_stones_1;
                            let mut hit_stones_2_clone_1 = hit_stones_2;
                            hit_stones_2_clone_1 = hit_stones_2_clone_1 - 1;
                            if (board_clone[(dice.1 - 1) as usize].0 == 1 && board_clone[(dice.1 - 1) as usize].1 == 1){
                                hit_stones_1_clone_1 = hit_stones_1_clone_1 + 1;
                            }
                            board_clone_1[(dice.1 - 1) as usize].0 = 2;
                            board_clone_1[(dice.1 - 1) as usize].1 = 1;
                            let mut temporary_v: Vec<i32> = vec![];
                            for i in 0..=23{
                                if board[i].0 == 2 {
                                    temporary_v.push(i as i32);
                                }
                            }
                            for i in temporary_v{
                                if (i + dice.0) <= 23{
                                    let mut board_clone_r = board_clone_1.clone();
                                    let mut hit_stones_1_clone_r = hit_stones_1_clone_1;    
                                    if !(board_clone_r[(i + dice.0) as usize].0 == 1 && board_clone_r[(i + dice.0) as usize].1 >= 2){
                                        if(board_clone_r[(i + dice.0) as usize].0 == 1 && board_clone_r[(i + dice.0) as usize].1 == 1){
                                            hit_stones_1_clone_r = hit_stones_1_clone_r + 1;
                                            board_clone_r[(i + dice.0) as usize].0 = 0;
                                            board_clone_r[(i + dice.0) as usize].1 = 0;
                                        }

                                        board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                        if(board_clone_r[i as usize].1 == 0){
                                            board_clone_r[i as usize].0 = 0;
                                        }

                                        board_clone_r[(i + dice.0) as usize].0 = 2;
                                        board_clone_r[(i + dice.0) as usize].1 = board_clone_r[(i + dice.0) as usize].1 + 1;
                                        set_board.insert(
                                            board_state { board: board_clone_r.clone(), 
                                                keep_count_1: keep_count_of_1_stones, 
                                                keep_count_2: keep_count_of_2_stones,
                                                hit_stones_1: hit_stones_1_clone_r, 
                                                hit_stones_2: hit_stones_2_clone_1,
                                                dice: dice.clone(),
                                                parent_index: -1
                                            }
                                        );
                                    }
                                }
                            } 
                        }
                    // more than 1 hit stone for player 2
                    }else{
                        if !(board_clone[(dice.0 -1) as usize].0 == 1 && board_clone[(dice.0 -1) as usize].1 >= 2){
                            hit_stones_2_clone = hit_stones_2_clone - 1;
                            if (board_clone[(dice.0 -1) as usize].0 == 1 && board_clone[(dice.0 -1) as usize].1 == 1){
                                hit_stones_1_clone = hit_stones_1_clone + 1;
                                board_clone[(dice.0 - 1) as usize].0 = 0;
                                board_clone[(dice.0 - 1) as usize].1 = 0;
                            }
                            board_clone[(dice.0 - 1) as usize].0 = 2;
                            board_clone[(dice.0 - 1) as usize].1 = board_clone[(dice.0 - 1) as usize].1 + 1;
                        }

                        if !(board[(dice.1 - 1) as usize].0 == 1 && board[(dice.1 - 1) as usize].1 >= 2){
                            hit_stones_2_clone = hit_stones_2_clone - 1;
                            if (board_clone[(dice.1 - 1) as usize].0 == 1 && board_clone[(dice.1 - 1) as usize].1 == 1){
                                hit_stones_1_clone = hit_stones_1_clone + 1;
                                board_clone[(dice.1 - 1) as usize].0 = 0;
                                board_clone[(dice.1 - 1) as usize].1 = 0;
                            }
                            board_clone[(dice.1 - 1) as usize].0 = 2;
                            board_clone[(dice.1 - 1) as usize].1 = board_clone[(dice.1 - 1) as usize].1 + 1;
                        }
                        set_board.insert(
                            board_state { board: board_clone.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                 hit_stones_1: hit_stones_1_clone, 
                                 hit_stones_2: hit_stones_2_clone,
                                dice: dice.clone(),
                                parent_index: -1 
                            }
                        );
                        
                    }
                    // for i in temporary_v{
                    // }
                    // Check if there i
                    println!("Size hashset: {}", set_board.len());
                    for mut s in set_board{
                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                    }

                // End of hit 2

                // Begining of turn 2, not alike dices
                // 
                }else{
                    // Indices from the board
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    let mut temporary_v: Vec<i32> = vec![];
                    for i in 0..=23{
                        if board[i].0 == 2 {
                            temporary_v.push(i as i32);
                        }
                    }
                    println!("{:?}", temporary_v);
                    // first usign the first dice
                    // What is happening with the board_clone
                    // basically we are going to have a bunch of these clones pushed right?
                    // we need to calculate the rest incuding itself
                    // I think we are good to go into the next phase
                    // which is changing the rest of the board with the second dice
                    // we also need to flip the order 

                    for i in temporary_v{
                        let mut board_clone = board.clone();
                        let mut hit_stones_1_clone = hit_stones_1;    
                        let mut keep_count_2_clone = keep_count_of_2_stones;
                        // Going out
                        // we need to check if everything is in zone of going out
                        let mut all_in_zone = 1;
                        for k in 0..=17{
                            if(board_clone[k].0 == 1){
                                all_in_zone = 0;
                            }
                        }

                        // is this fine?
                        if i + dice.0 > 23 && all_in_zone == 1{
                            keep_count_2_clone = keep_count_2_clone - 1;
                            board_clone[i as usize].1 = board_clone[i as usize].1 - 1;
                            if board_clone[i as usize].1 == 0 {
                                board_clone[i as usize].0 = 0
                            }
                            
                        // Moving on the board
                        } else{
                            if i + dice.0 <= 23{
                                if !(board_clone[(i + dice.0) as usize].0 == 1 && board_clone[(i + dice.0) as usize].1 > 1){
                                    if(board_clone[(i + dice.0) as usize].0 == 1 && board_clone[(i + dice.0) as usize].1 == 1){
                                        hit_stones_1_clone = hit_stones_1_clone + 1;
                                        board_clone[(i + dice.0) as usize].0 = 0;
                                        board_clone[(i + dice.0) as usize].1 = 0;
                                    }

                                    board_clone[i as usize].1 = board_clone[i as usize].1 - 1;
                                    if(board_clone[i as usize].1 == 0){
                                        board_clone[i as usize].0 = 0;
                                    }
                                    board_clone[(i + dice.0) as usize].0 = 2;
                                    board_clone[(i + dice.0) as usize].1 = board_clone[(i + dice.0) as usize].1 + 1;
                                    let mut remaining: Vec<i32> = vec![];
                                    for j in 0..=i{
                                        if (board_clone[j as usize].0 == 2){
                                            remaining.push(j);
                                        }
                                    }
                                    for r in remaining{
                                        let mut board_clone_r = board_clone.clone();
                                        let mut hit_stones_1_clone_r = hit_stones_1_clone;    
                                        let mut keep_count_2_clone_r = keep_count_2_clone;
                                        let mut all_in_zone_r = 1;

                                        for k in 0..=17{
                                            if(board_clone_r[k].0 == 1){
                                                all_in_zone_r = 0;
                                            }
                                        }

                                        if r - dice.1 < 0 && all_in_zone_r == 1{
                                            keep_count_2_clone_r = keep_count_2_clone_r - 1;
                                            board_clone_r[i as usize].1 = board_clone_r[i as usize].1 - 1;
                                            if board_clone_r[i as usize].1 == 0 {
                                                board_clone_r[i as usize].0 = 0
                                            }
                                            set_board.insert(
                                                board_state{
                                                    board: board_clone_r.clone(),
                                                    keep_count_1:  keep_count_of_1_stones,
                                                    keep_count_2:  keep_count_2_clone_r,
                                                    hit_stones_1: hit_stones_1_clone_r,
                                                    hit_stones_2: hit_stones_2,
                                                    dice: dice.clone(), 
                                                    parent_index: -1
                                                }
                                            );
                                        // Moving on the board
                                        } else{
                                            if r + dice.1 <= 23 {
                                                if !(board_clone[(r + dice.1) as usize].0 == 1 && board_clone[(r + dice.1) as usize].1 > 1){
                                                    if(board_clone[(r + dice.1) as usize].0 == 1 && board_clone[(r + dice.1) as usize].1 == 1){
                                                        hit_stones_1_clone_r = hit_stones_1_clone_r + 1;
                                                        board_clone[(r + dice.1) as usize].0 = 0;
                                                        board_clone[(r + dice.1) as usize].1 = 0;
                                                    }
                                                    board_clone_r[r as usize].1 = board_clone_r[r as usize].1 - 1;
                                                    if(board_clone_r[r as usize].1 == 0){
                                                        board_clone_r[r as usize].0 = 0;
                                                    }
                                                    board_clone_r[(r + dice.1) as usize].0 = 2;
                                                    board_clone_r[(r + dice.1) as usize].1 = board_clone_r[(r + dice.1) as usize].1 + 1;
                                                    set_board.insert(
                                                        board_state{
                                                            board: board_clone_r.clone(),
                                                            keep_count_1: keep_count_of_1_stones,
                                                            keep_count_2:  keep_count_2_clone_r,
                                                            hit_stones_1: hit_stones_1_clone_r,
                                                            hit_stones_2: hit_stones_2,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                }
                                            }
                                        }
                                    } 
                                }
                            }
                        }
                    }
                    read_input();
                    println!("Size hashset: {}", set_board.len());
                    for mut s in set_board{
                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                    }
                }
            }

        // else for the dices not being the same
        } else {
            if turn == 1{
                if hit_stones_1 > 0{
                    println!("HIT_STONE_1 IN DICE.0 == DICE.1");
                    let mut board_clone_1: Vec<(i32, i32)> = board.clone();
                    let mut hit_stones_2_clone_1 = hit_stones_2;    
                    let mut hit_stones_1_clone = hit_stones_1;
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    if !(board[(24 - dice.0) as usize].0 == 2 && board[(24 - dice.0) as usize].1 >= 2) {
                        // how can I fix the 
                        // What are we doing ? we are basically
                        let mut remaining_dice = 0;
                        if(board_clone_1[(24 - dice.0) as usize].0 == 2 && board_clone_1[(24 - dice.0 ) as usize].1 == 1){
                            hit_stones_2_clone_1 = hit_stones_2_clone_1 + 1;


                        }
                        if(hit_stones_1_clone > 4){
                            hit_stones_1_clone = hit_stones_1_clone - 4;
                            board_clone_1[(24 - dice.0) as usize].0 = 1;
                            board_clone_1[(24 - dice.0) as usize].1 = 4;
                            set_board.insert(                                                
                                board_state { board: board_clone_1.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                hit_stones_1: hit_stones_1_clone, 
                                hit_stones_2: hit_stones_2_clone_1,
                                dice: dice.clone(),
                                parent_index: -1
                            }
                            );
                            // if no remaining dice then the board is just added here

                        } else if hit_stones_1_clone == 4 {
                            board_clone_1[(24 - dice.0) as usize].0 = 1;
                            board_clone_1[(24 - dice.0) as usize].1 = 4;
                            hit_stones_1_clone = 0;
                            set_board.insert(                                                
                                board_state { board: board_clone_1.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                hit_stones_1: hit_stones_1_clone, 
                                hit_stones_2: hit_stones_2_clone_1,
                                dice: dice.clone(),
                                parent_index: -1
                            }
                            );

                        } else {
                            remaining_dice = 4 - hit_stones_1_clone;
                            board_clone_1[(24 - dice.0) as usize].0 = 1;
                            board_clone_1[(24 - dice.0) as usize].1 = hit_stones_1_clone;
                            hit_stones_1_clone = 0;
                            
                            let mut one_move_set: HashSet<board_state> = HashSet::new();
                            let mut two_move_set: HashSet<board_state> = HashSet::new();
                            let mut three_move_set: HashSet<board_state> = HashSet::new();
                            println!("REMAINING DICE: {}", remaining_dice);
                            read_input();
                            for i in (0..=23).rev(){
                                if(board[i as usize].0 == 1){
                                    let mut board_clone_i: Vec<(i32, i32)> = board_clone_1.clone();
                                    let mut hit_stones_2_clone_i = hit_stones_2;    
                                    let mut keep_count_1_clone_i = keep_count_of_1_stones;
                                    // Going out
                                    // we need to check if everything is in zone of going out
                                    let mut all_in_zone = 1;
                                    let mut did_move_in_i = 0;
                                    for r in 6..=23{
                                        if(board_clone_i[r].0 == 1){
                                            all_in_zone = 0;
                                        }
                                    }
                                    if i - dice.0 < 0 && all_in_zone == 1{
                                        keep_count_1_clone_i = keep_count_1_clone_i - 1;
                                        board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                        if board_clone_i[i as usize].1 == 0 {
                                            board_clone_i[i as usize].0 = 0
                                        }
                                        did_move_in_i = 1;
                                        one_move_set.insert(
                                            board_state { board: board_clone_i.clone(), 
                                                keep_count_1: keep_count_1_clone_i, 
                                                keep_count_2: keep_count_of_2_stones,
                                                hit_stones_1: hit_stones_1_clone, 
                                                hit_stones_2: hit_stones_2_clone_i,
                                                dice: dice.clone(),
                                                parent_index: -1
                                            }
                                        );
                                    // Moving on the board
                                    } else{
                                        if i - dice.0 >= 0{
                                            if !(board_clone_i[(i - dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 > 1){
                                                if(board_clone_i[(i- dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 == 1){
                                                    hit_stones_2_clone_i = hit_stones_2_clone_i + 1;
                                                    board_clone_i[(i - dice.0) as usize].0 = 0;
                                                    board_clone_i[(i - dice.0) as usize].1 = 0;
                                                }
                                                board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                                if(board_clone_i[i as usize].1 == 0){
                                                    board_clone_i[i as usize].0 = 0;
                                                }
                                                board_clone_i[(i - dice.0) as usize].0 = 1;
                                                board_clone_i[(i - dice.0) as usize].1 = board_clone_i[(i - dice.0) as usize].1 + 1;
        
                                                did_move_in_i = 1;
                                                one_move_set.insert(
                                                    board_state { board: board_clone_i.clone(), 
                                                        keep_count_1: keep_count_1_clone_i, 
                                                        keep_count_2: keep_count_of_2_stones,
                                                        hit_stones_1: hit_stones_1_clone, 
                                                        hit_stones_2: hit_stones_2_clone_i,
                                                        dice: dice.clone(),
                                                        parent_index: -1
                                                    }
                                                );
                                            }
                                        }
                                    }
        
                                    // println!("i: {}", i);
                                    // for mut s in &one_move_set{
                                    //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                    // }
                                    //read_input();
                                    
                                    if did_move_in_i == 1 && remaining_dice > 1{
                                        for j in (0..=i).rev(){
                                            if(board_clone_i[j as usize].0 == 1){
                                                let mut board_clone_j: Vec<(i32, i32)> = board_clone_i.clone();
                                                let mut hit_stones_2_clone_j = hit_stones_2_clone_i;    
                                                let mut keep_count_1_clone_j = keep_count_1_clone_i;
                                                // Going out
                                                // we need to check if everything is in zone of going out
                                                let mut did_move_in_j = 0;
                                                let mut all_in_zone = 1;
                                                for r in 6..=23{
                                                    if(board_clone_j[r].0 == 1){
                                                        all_in_zone = 0;
                                                    }
                                                }
                                                if j - dice.0 < 0 && all_in_zone == 1{
                                                    keep_count_1_clone_j = keep_count_1_clone_j - 1;
                                                    board_clone_j[i as usize].1 = board_clone_j[i as usize].1 - 1;
                                                    if board_clone_j[i as usize].1 == 0 {
                                                        board_clone_j[i as usize].0 = 0
                                                    }
                                                    did_move_in_j = 1;
                                                    two_move_set.insert(
                                                        board_state { board: board_clone_j.clone(), 
                                                            keep_count_1: keep_count_1_clone_j, 
                                                            keep_count_2: keep_count_of_2_stones,
                                                            hit_stones_1: hit_stones_1_clone, 
                                                            hit_stones_2: hit_stones_2_clone_j,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                // Moving on the board
                                                } else{
                                                    if j - dice.0 >= 0{
                                                        if !(board_clone_j[(j - dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 > 1){
                                                            if(board_clone_j[(j- dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 == 1){
                                                                hit_stones_2_clone_j = hit_stones_2_clone_j + 1;
                                                                board_clone_j[(j - dice.0) as usize].0 = 0;
                                                                board_clone_j[(j - dice.0) as usize].1 = 0;
                                                            }
                                                            board_clone_j[j as usize].1 = board_clone_j[j as usize].1 - 1;
                                                            if(board_clone_j[j as usize].1 == 0){
                                                                board_clone_j[j as usize].0 = 0;
                                                            }
                                                            board_clone_j[(j - dice.0) as usize].0 = 1;
                                                            board_clone_j[(j - dice.0) as usize].1 = board_clone_j[(j - dice.0) as usize].1 + 1;
                                                            did_move_in_j = 1;
                                                            two_move_set.insert(
                                                                board_state { board: board_clone_j.clone(), 
                                                                    keep_count_1: keep_count_1_clone_j, 
                                                                    keep_count_2: keep_count_of_2_stones,
                                                                    hit_stones_1: hit_stones_1_clone, 
                                                                    hit_stones_2: hit_stones_2_clone_j,
                                                                    dice: dice.clone(),
                                                                    parent_index: -1
                                                                }
                                                            );
                                                            // println!("j: {}", j);
                                                            // for mut s in &two_move_set{
                                                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                            // }
                                                            //read_input();
                                                        }
                                                    }
                                                }
                                                if did_move_in_j == 1 && remaining_dice > 2{
                                                    for k in (0..=j).rev(){
                                                        if(board_clone_j[k as usize].0 ==1){
                                                            let mut board_clone_k = board_clone_j.clone();
                                                            let mut hit_stones_2_clone_k = hit_stones_2_clone_j;    
                                                            let mut keep_count_1_clone_k = keep_count_1_clone_j;
                                                            // Going out
                                                            // we need to check if everything is in zone of going out
                                                            
                                                            let mut all_in_zone = 1;
                                                            for r in 6..=23{
                                                                if(board_clone_j[r].0 == 1){
                                                                    all_in_zone = 0;
                                                                }
                                                            }
                                                            if k - dice.0 < 0 && all_in_zone == 1{
                                                                keep_count_1_clone_k = keep_count_1_clone_k - 1;
                                                                board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                if board_clone_k[k as usize].1 == 0 {
                                                                    board_clone_k[k as usize].0 = 0
                                                                }
                                                                three_move_set.insert(
                                                                    board_state { board: board_clone_k.clone(), 
                                                                        keep_count_1: keep_count_1_clone_k, 
                                                                        keep_count_2: keep_count_of_2_stones,
                                                                        hit_stones_1: hit_stones_1_clone, 
                                                                        hit_stones_2: hit_stones_2_clone_k,
                                                                        dice: dice.clone(),
                                                                        parent_index: -1
                                                                    }
                                                                );
                                                            // Moving on the board
                                                            } else{
                                                                if k - dice.0 >= 0{
                                                                    if !(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 > 1){
                                                                        if(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 == 1){
                                                                            hit_stones_2_clone_k = hit_stones_2_clone_k + 1;
                                                                            board_clone_k[(k - dice.0) as usize].0 = 0;
                                                                            board_clone_k[(k - dice.0) as usize].1 = 0;
                                                                        }
                                                                        board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                        if(board_clone_k[k as usize].1 == 0){
                                                                            board_clone_k[k as usize].0 = 0;
                                                                        }
                                                                        board_clone_k[(k - dice.0) as usize].0 = 1;
                                                                        board_clone_k[(k - dice.0) as usize].1 = board_clone_k[(k - dice.0) as usize].1 + 1;
                                                                        three_move_set.insert(
                                                                            board_state { board: board_clone_k.clone(), 
                                                                                keep_count_1: keep_count_1_clone_k, 
                                                                                keep_count_2: keep_count_of_2_stones,
                                                                                hit_stones_1: hit_stones_1_clone, 
                                                                                hit_stones_2: hit_stones_2_clone_k,
                                                                                dice: dice.clone(),
                                                                                parent_index: -1
                                                                            }
                                                                        );
                                                                        // println!("k: {}", k);
                                                                        // for mut s in &three_move_set{
                                                                        //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                        // }
                                                                        //read_input();
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            println!("{}", three_move_set.len());
                            println!("{}", two_move_set.len());
                            println!("{}", one_move_set.len());
                            read_input();
                            if three_move_set.len() != 0{
                                println!("Three");
                                for mut s in set_board{
                                    display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                }
                                set_board = three_move_set.clone();
                            }else{
                                println!("we should be here");
                                if two_move_set.len() != 0{
                                    println!("Two");
                                    set_board = two_move_set.clone();
                                    for mut s in set_board{
                                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                    }
                                    
                                }else{
                                    if one_move_set.len() != 0{
                                        println!("One");
                                        set_board = one_move_set.clone();
                                        for mut s in set_board{
                                            display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                        }
                                        
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("Here in dice.0 == dice.1");
                    read_input();
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    // Find an anchor 
                    // This is supposed to figure out the 4 possible moves
                    // if not 3 possible moves
                    // if not 2 possible moves
                    // if not 1  possible moves
                    let mut one_move_set: HashSet<board_state> = HashSet::new();
                    let mut two_move_set: HashSet<board_state> = HashSet::new();
                    let mut three_move_set: HashSet<board_state> = HashSet::new();
                    let mut four_move_set: HashSet<board_state> = HashSet::new();
                    // looping over the 
                    // I don't think we need these hashsets because we can
                    // just loop over stuff and be fine with stuff
                    
                    for i in (0..=23).rev(){
                        if(board[i as usize].0 == 1){
                            let mut board_clone_i: Vec<(i32, i32)> = board.clone();
                            let mut hit_stones_2_clone_i = hit_stones_2;    
                            let mut keep_count_1_clone_i = keep_count_of_1_stones;
                            // Going out
                            // we need to check if everything is in zone of going out
                            let mut all_in_zone = 1;
                            let mut did_move_in_i = 0;
                            for r in 6..=23{
                                if(board_clone_i[r].0 == 1){
                                    all_in_zone = 0;
                                }
                            }
                            if i - dice.0 < 0 && all_in_zone == 1{
                                keep_count_1_clone_i = keep_count_1_clone_i - 1;
                                board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                if board_clone_i[i as usize].1 == 0 {
                                    board_clone_i[i as usize].0 = 0
                                }
                                did_move_in_i = 1;
                                one_move_set.insert(
                                    board_state { board: board_clone_i.clone(), 
                                        keep_count_1: keep_count_1_clone_i, 
                                        keep_count_2: keep_count_of_2_stones,
                                         hit_stones_1: hit_stones_1, 
                                         hit_stones_2: hit_stones_2_clone_i,
                                        dice: dice.clone(),
                                        parent_index: -1
                                    }
                                );
                            // Moving on the board
                            } else{
                                if i - dice.0 >= 0{
                                    if !(board_clone_i[(i - dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 > 1){
                                        if(board_clone_i[(i- dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 == 1){
                                            hit_stones_2_clone_i = hit_stones_2_clone_i + 1;
                                            board_clone_i[(i - dice.0) as usize].0 = 0;
                                            board_clone_i[(i - dice.0) as usize].1 = 0;
                                        }
                                        board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                        if(board_clone_i[i as usize].1 == 0){
                                            board_clone_i[i as usize].0 = 0;
                                        }
                                        board_clone_i[(i - dice.0) as usize].0 = 1;
                                        board_clone_i[(i - dice.0) as usize].1 = board_clone_i[(i - dice.0) as usize].1 + 1;

                                        did_move_in_i = 1;
                                        one_move_set.insert(
                                            board_state { board: board_clone_i.clone(), 
                                                keep_count_1: keep_count_1_clone_i, 
                                                keep_count_2: keep_count_of_2_stones,
                                                 hit_stones_1: hit_stones_1, 
                                                 hit_stones_2: hit_stones_2_clone_i,
                                                dice: dice.clone(),
                                                parent_index: -1
                                            }
                                        );
                                    }
                                }
                            }

                            // println!("i: {}", i);
                            // for mut s in &one_move_set{
                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                            // }
                            //read_input();
                            if did_move_in_i == 1{
                                for j in (0..=i).rev(){
                                    if(board_clone_i[j as usize].0 == 1){
                                        let mut board_clone_j: Vec<(i32, i32)> = board_clone_i.clone();
                                        let mut hit_stones_2_clone_j = hit_stones_2_clone_i;    
                                        let mut keep_count_1_clone_j = keep_count_1_clone_i;
                                        // Going out
                                        // we need to check if everything is in zone of going out
                                        let mut did_move_in_j = 0;
                                        let mut all_in_zone = 1;
                                        for r in 6..=23{
                                            if(board_clone_j[r].0 == 1){
                                                all_in_zone = 0;
                                            }
                                        }
                                        if j - dice.0 < 0 && all_in_zone == 1{
                                            keep_count_1_clone_j = keep_count_1_clone_j - 1;
                                            board_clone_j[i as usize].1 = board_clone_j[i as usize].1 - 1;
                                            if board_clone_j[i as usize].1 == 0 {
                                                board_clone_j[i as usize].0 = 0
                                            }
                                            did_move_in_j = 1;
                                            two_move_set.insert(
                                                board_state { board: board_clone_j.clone(), 
                                                    keep_count_1: keep_count_1_clone_j, 
                                                    keep_count_2: keep_count_of_2_stones,
                                                     hit_stones_1: hit_stones_1, 
                                                     hit_stones_2: hit_stones_2_clone_j,
                                                    dice: dice.clone(),
                                                    parent_index: -1
                                                }
                                            );
                                        // Moving on the board
                                        } else{
                                            if j - dice.0 >= 0{
                                                if !(board_clone_j[(j - dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 > 1){
                                                    if(board_clone_j[(j- dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 == 1){
                                                        hit_stones_2_clone_j = hit_stones_2_clone_j + 1;
                                                        board_clone_j[(j - dice.0) as usize].0 = 0;
                                                        board_clone_j[(j - dice.0) as usize].1 = 0;
                                                    }
                                                    board_clone_j[j as usize].1 = board_clone_j[j as usize].1 - 1;
                                                    if(board_clone_j[j as usize].1 == 0){
                                                        board_clone_j[j as usize].0 = 0;
                                                    }
                                                    board_clone_j[(j - dice.0) as usize].0 = 1;
                                                    board_clone_j[(j - dice.0) as usize].1 = board_clone_j[(j - dice.0) as usize].1 + 1;
                                                    did_move_in_j = 1;
                                                    two_move_set.insert(
                                                        board_state { board: board_clone_j.clone(), 
                                                            keep_count_1: keep_count_1_clone_j, 
                                                            keep_count_2: keep_count_of_2_stones,
                                                             hit_stones_1: hit_stones_1, 
                                                             hit_stones_2: hit_stones_2_clone_j,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                    // println!("j: {}", j);
                                                    // for mut s in &two_move_set{
                                                    //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                    // }
                                                    //read_input();
                                                }
                                            }
                                        }
                                        if did_move_in_j == 1{
                                            for k in (0..=j).rev(){
                                                if(board_clone_j[k as usize].0 ==1){
                                                    let mut board_clone_k = board_clone_j.clone();
                                                    let mut hit_stones_2_clone_k = hit_stones_2_clone_j;    
                                                    let mut keep_count_1_clone_k = keep_count_1_clone_j;
                                                    // Going out
                                                    // we need to check if everything is in zone of going out
                                                    let mut did_move_in_k = 0;
                                                    let mut all_in_zone = 1;
                                                    for r in 6..=23{
                                                        if(board_clone_j[r].0 == 1){
                                                            all_in_zone = 0;
                                                        }
                                                    }
                                                    if k - dice.0 < 0 && all_in_zone == 1{
                                                        keep_count_1_clone_k = keep_count_1_clone_k - 1;
                                                        board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                        if board_clone_k[k as usize].1 == 0 {
                                                            board_clone_k[k as usize].0 = 0
                                                        }
                                                        did_move_in_k = 1;
                                                        three_move_set.insert(
                                                            board_state { board: board_clone_k.clone(), 
                                                                keep_count_1: keep_count_1_clone_k, 
                                                                keep_count_2: keep_count_of_2_stones,
                                                                 hit_stones_1: hit_stones_1, 
                                                                 hit_stones_2: hit_stones_2_clone_k,
                                                                dice: dice.clone(),
                                                                parent_index: -1
                                                            }
                                                        );
                                                    // Moving on the board
                                                    } else{
                                                        if k - dice.0 >= 0{
                                                            if !(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 > 1){
                                                                if(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 == 1){
                                                                    hit_stones_2_clone_k = hit_stones_2_clone_k + 1;
                                                                    board_clone_k[(k - dice.0) as usize].0 = 0;
                                                                    board_clone_k[(k - dice.0) as usize].1 = 0;
                                                                }
                                                                board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                if(board_clone_k[k as usize].1 == 0){
                                                                    board_clone_k[k as usize].0 = 0;
                                                                }
                                                                board_clone_k[(k - dice.0) as usize].0 = 1;
                                                                board_clone_k[(k - dice.0) as usize].1 = board_clone_k[(k - dice.0) as usize].1 + 1;
                                                                did_move_in_k = 1;
                                                                three_move_set.insert(
                                                                    board_state { board: board_clone_k.clone(), 
                                                                        keep_count_1: keep_count_1_clone_k, 
                                                                        keep_count_2: keep_count_of_2_stones,
                                                                         hit_stones_1: hit_stones_1, 
                                                                         hit_stones_2: hit_stones_2_clone_k,
                                                                        dice: dice.clone(),
                                                                        parent_index: -1
                                                                    }
                                                                );
                                                                // println!("k: {}", k);
                                                                // for mut s in &three_move_set{
                                                                //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                // }
                                                                //read_input();
                                                            }
                                                        }
                                                    }
                                                    if did_move_in_k == 1{
                                                        for l in (0..=k).rev(){
                                                            if(board_clone_k[l as usize].0 == 1){
                                                                let mut board_clone_l = board_clone_k.clone();
                                                                let mut hit_stones_2_clone_l = hit_stones_2_clone_k;    
                                                                let mut keep_count_1_clone_l = keep_count_1_clone_k;
                                                                // we need to check if everything is in zone of going out
                                                                let mut all_in_zone = 1;
                                                                for r in 6..=23{
                                                                    if(board_clone_k[r].0 == 1){
                                                                        all_in_zone = 0;
                                                                    }
                                                                }
                                                                if l - dice.0 < 0 && all_in_zone == 1{
                                                                    keep_count_1_clone_l = keep_count_1_clone_l - 1;
                                                                    board_clone_l[l as usize].1 = board_clone_l[l as usize].1 - 1;
                                                                    if board_clone_l[l as usize].1 == 0 {
                                                                        board_clone_l[l as usize].0 = 0
                                                                    }
                                                                    four_move_set.insert(
                                                                        board_state { board: board_clone_l.clone(), 
                                                                            keep_count_1: keep_count_1_clone_l, 
                                                                            keep_count_2: keep_count_of_2_stones,
                                                                             hit_stones_1: hit_stones_1, 
                                                                             hit_stones_2: hit_stones_2_clone_l,
                                                                            dice: dice.clone(),
                                                                            parent_index: -1
                                                                        }
                                                                    );
                                                                // Moving on the board
                                                                } else{
                                                                    if l - dice.0 >= 0{
                                                                        if !(board_clone_l[(l - dice.0) as usize].0 == 2 && board_clone_l[(l - dice.0) as usize].1 > 1){
                                                                            if(board_clone_l[(l - dice.0) as usize].0 == 2 && board_clone_l[(l - dice.0) as usize].1 == 1){
                                                                                hit_stones_2_clone_l = hit_stones_2_clone_l + 1;
                                                                                board_clone_l[(l - dice.0) as usize].0 = 0;
                                                                                board_clone_l[(l - dice.0) as usize].1 = 0;
                                                                            }
                                                                            board_clone_l[l as usize].1 = board_clone_l[l as usize].1 - 1;
                                                                            if(board_clone_l[l as usize].1 == 0){
                                                                                board_clone_l[l as usize].0 = 0;
                                                                            }
                                                                            board_clone_l[(l - dice.0) as usize].0 = 1;
                                                                            board_clone_l[(l - dice.0) as usize].1 = board_clone_l[(l - dice.0) as usize].1 + 1;
                                                                            four_move_set.insert(
                                                                                board_state { board: board_clone_l.clone(), 
                                                                                    keep_count_1: keep_count_1_clone_l, 
                                                                                    keep_count_2: keep_count_of_2_stones,
                                                                                    hit_stones_1: hit_stones_1, 
                                                                                    hit_stones_2: hit_stones_2_clone_l,
                                                                                    dice: dice.clone(),
                                                                                    parent_index: -1
                                                                                }
                                                                            );
                                                                            
                                                                            // println!("l: {}", l);
                                                                            // for mut s in &four_move_set{
                                                                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                            // }
                                                                            // println!("LINE 3253");
                                                                            // read_input();
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    println!("{}", four_move_set.len());
                    println!("{}", three_move_set.len());
                    println!("{}", two_move_set.len());
                    println!("{}", one_move_set.len());
                    read_input();
                    if four_move_set.len() != 0{
                        set_board = four_move_set.clone();
                        for mut s in set_board{
                            println!("Four");
                            display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                        }
                    }else {
                        if three_move_set.len() != 0{
                            println!("Three");
                            for mut s in set_board{
                                display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                            }
                            set_board = three_move_set.clone();
                        }else{
                            if two_move_set.len() != 0{
                                println!("Two");
                                for mut s in set_board{
                                    display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                }
                                set_board = two_move_set.clone();
                            }else{
                                if one_move_set.len() != 0{
                                    println!("One");
                                    for mut s in set_board{
                                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                    }
                                    set_board = one_move_set.clone();
                                }
                            }
                        }
                    }
                }
            } else {
                // ==========================================================
                // ======================== TURN 2 ==========================
                // ===================== DICES BEING EQUAL  =================
                // ==========================================================
                if hit_stones_2 > 0{
                    println!("HIT_STONE_2 IN DICE.0 == DICE.1");

                    let mut board_clone_2: Vec<(i32, i32)> = board.clone();
                    let mut hit_stones_1_clone_2 = hit_stones_1;    
                    let mut hit_stones_2_clone = hit_stones_2;
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    if !(board[(dice.0 - 1) as usize].0 == 1 && board[(dice.0 -1) as usize].1 >= 2) {
                        // how can I fix the 
                        // What are we doing ? we are basically
                        let mut remaining_dice = 0;
                        if(board_clone_2[(dice.0 - 1) as usize].0 == 1 && board_clone_2[(dice.0 - 1) as usize].1 == 1){
                            hit_stones_1_clone_2 = hit_stones_1_clone_2 + 1;


                        }
                        if(hit_stones_2_clone > 4){
                            hit_stones_2_clone = hit_stones_2_clone - 4;
                            board_clone_2[(dice.0 - 1) as usize].0 = 1;
                            board_clone_2[(dice.0 - 1) as usize].1 = 4;
                            set_board.insert(                                                
                                board_state { board: board_clone_2.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                hit_stones_1: hit_stones_1_clone_2,
                                hit_stones_2: hit_stones_2_clone, 
                                dice: dice.clone(),
                                parent_index: -1
                            }
                            );
                            // if no remaining dice then the board is just added here

                        } else if hit_stones_2_clone == 4 {
                            board_clone_2[(dice.0 - 1) as usize].0 = 1;
                            board_clone_2[(dice.0 - 1) as usize].1 = 4;
                            hit_stones_2_clone = 0;
                            set_board.insert(                                                
                                board_state { board: board_clone_2.clone(), 
                                keep_count_1: keep_count_of_1_stones, 
                                keep_count_2: keep_count_of_2_stones,
                                hit_stones_1: hit_stones_1_clone_2, 
                                hit_stones_2: hit_stones_2_clone, 
                                dice: dice.clone(),
                                parent_index: -1
                            }
                            );

                        } else {
                            remaining_dice = 4 - hit_stones_2_clone;
                            board_clone_2[(dice.0 - 1) as usize].0 = 1;
                            board_clone_2[(dice.0 - 1) as usize].1 = hit_stones_2_clone;
                            hit_stones_2_clone = 0;
                            
                            let mut one_move_set: HashSet<board_state> = HashSet::new();
                            let mut two_move_set: HashSet<board_state> = HashSet::new();
                            let mut three_move_set: HashSet<board_state> = HashSet::new();
                            println!("REMAINING DICE: {}", remaining_dice);
                            read_input();
                            for i in (0..=23).rev(){
                                if(board[i as usize].0 == 1){
                                    let mut board_clone_i: Vec<(i32, i32)> = board_clone_2.clone();
                                    let mut hit_stones_1_clone_i = hit_stones_1;    
                                    let mut keep_count_2_clone_i = keep_count_of_2_stones;
                                    // Going out
                                    // we need to check if everything is in zone of going out
                                    let mut all_in_zone = 1;
                                    let mut did_move_in_i = 0;
                                    for r in 6..=23{
                                        if(board_clone_i[r].0 == 1){
                                            all_in_zone = 0;
                                        }
                                    }

                                    if i - dice.0 <= 23{
                                        if !(board_clone_i[(i + dice.0) as usize].0 == 2 && board_clone_i[(i + dice.0) as usize].1 > 1){
                                            if(board_clone_i[(i + dice.0) as usize].0 == 2 && board_clone_i[(i + dice.0) as usize].1 == 1){
                                                hit_stones_1_clone_i = hit_stones_1_clone_i + 1;
                                                board_clone_i[(i - dice.0) as usize].0 = 0;
                                                board_clone_i[(i - dice.0) as usize].1 = 0;
                                            }
                                            board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                            if(board_clone_i[i as usize].1 == 0){
                                                board_clone_i[i as usize].0 = 0;
                                            }
                                            board_clone_i[(i - dice.0) as usize].0 = 1;
                                            board_clone_i[(i - dice.0) as usize].1 = board_clone_i[(i - dice.0) as usize].1 + 1;
    
                                            did_move_in_i = 1;
                                            one_move_set.insert(
                                                board_state { board: board_clone_i.clone(), 
                                                    keep_count_1: keep_count_1_clone_i, 
                                                    keep_count_2: keep_count_of_2_stones,
                                                    hit_stones_1: hit_stones_1_clone, 
                                                    hit_stones_2: hit_stones_2_clone_i,
                                                    dice: dice.clone(),
                                                    parent_index: -1
                                                }
                                            );
                                        }
                                    }
                                    
        
                                    // println!("i: {}", i);
                                    // for mut s in &one_move_set{
                                    //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                    // }
                                    //read_input();
                                    
                                    if did_move_in_i == 1 && remaining_dice > 1{
                                        for j in (0..=i).rev(){
                                            if(board_clone_i[j as usize].0 == 1){
                                                let mut board_clone_j: Vec<(i32, i32)> = board_clone_i.clone();
                                                let mut hit_stones_2_clone_j = hit_stones_2_clone_i;    
                                                let mut keep_count_1_clone_j = keep_count_1_clone_i;
                                                // Going out
                                                // we need to check if everything is in zone of going out
                                                let mut did_move_in_j = 0;
                                                let mut all_in_zone = 1;
                                                for r in 6..=23{
                                                    if(board_clone_j[r].0 == 1){
                                                        all_in_zone = 0;
                                                    }
                                                }
                                                if j - dice.0 < 0 && all_in_zone == 1{
                                                    keep_count_1_clone_j = keep_count_1_clone_j - 1;
                                                    board_clone_j[i as usize].1 = board_clone_j[i as usize].1 - 1;
                                                    if board_clone_j[i as usize].1 == 0 {
                                                        board_clone_j[i as usize].0 = 0
                                                    }
                                                    did_move_in_j = 1;
                                                    two_move_set.insert(
                                                        board_state { board: board_clone_j.clone(), 
                                                            keep_count_1: keep_count_1_clone_j, 
                                                            keep_count_2: keep_count_of_2_stones,
                                                            hit_stones_1: hit_stones_1_clone, 
                                                            hit_stones_2: hit_stones_2_clone_j,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                // Moving on the board
                                                } else{
                                                    if j - dice.0 >= 0{
                                                        if !(board_clone_j[(j - dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 > 1){
                                                            if(board_clone_j[(j- dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 == 1){
                                                                hit_stones_2_clone_j = hit_stones_2_clone_j + 1;
                                                                board_clone_j[(j - dice.0) as usize].0 = 0;
                                                                board_clone_j[(j - dice.0) as usize].1 = 0;
                                                            }
                                                            board_clone_j[j as usize].1 = board_clone_j[j as usize].1 - 1;
                                                            if(board_clone_j[j as usize].1 == 0){
                                                                board_clone_j[j as usize].0 = 0;
                                                            }
                                                            board_clone_j[(j - dice.0) as usize].0 = 1;
                                                            board_clone_j[(j - dice.0) as usize].1 = board_clone_j[(j - dice.0) as usize].1 + 1;
                                                            did_move_in_j = 1;
                                                            two_move_set.insert(
                                                                board_state { board: board_clone_j.clone(), 
                                                                    keep_count_1: keep_count_1_clone_j, 
                                                                    keep_count_2: keep_count_of_2_stones,
                                                                    hit_stones_1: hit_stones_1_clone, 
                                                                    hit_stones_2: hit_stones_2_clone_j,
                                                                    dice: dice.clone(),
                                                                    parent_index: -1
                                                                }
                                                            );
                                                            // println!("j: {}", j);
                                                            // for mut s in &two_move_set{
                                                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                            // }
                                                            //read_input();
                                                        }
                                                    }
                                                }
                                                if did_move_in_j == 1 && remaining_dice > 2{
                                                    for k in (0..=j).rev(){
                                                        if(board_clone_j[k as usize].0 ==1){
                                                            let mut board_clone_k = board_clone_j.clone();
                                                            let mut hit_stones_2_clone_k = hit_stones_2_clone_j;    
                                                            let mut keep_count_1_clone_k = keep_count_1_clone_j;
                                                            // Going out
                                                            // we need to check if everything is in zone of going out
                                                            
                                                            let mut all_in_zone = 1;
                                                            for r in 6..=23{
                                                                if(board_clone_j[r].0 == 1){
                                                                    all_in_zone = 0;
                                                                }
                                                            }
                                                            if k - dice.0 < 0 && all_in_zone == 1{
                                                                keep_count_1_clone_k = keep_count_1_clone_k - 1;
                                                                board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                if board_clone_k[k as usize].1 == 0 {
                                                                    board_clone_k[k as usize].0 = 0
                                                                }
                                                                three_move_set.insert(
                                                                    board_state { board: board_clone_k.clone(), 
                                                                        keep_count_1: keep_count_1_clone_k, 
                                                                        keep_count_2: keep_count_of_2_stones,
                                                                        hit_stones_1: hit_stones_1_clone, 
                                                                        hit_stones_2: hit_stones_2_clone_k,
                                                                        dice: dice.clone(),
                                                                        parent_index: -1
                                                                    }
                                                                );
                                                            // Moving on the board
                                                            } else{
                                                                if k - dice.0 >= 0{
                                                                    if !(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 > 1){
                                                                        if(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 == 1){
                                                                            hit_stones_2_clone_k = hit_stones_2_clone_k + 1;
                                                                            board_clone_k[(k - dice.0) as usize].0 = 0;
                                                                            board_clone_k[(k - dice.0) as usize].1 = 0;
                                                                        }
                                                                        board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                        if(board_clone_k[k as usize].1 == 0){
                                                                            board_clone_k[k as usize].0 = 0;
                                                                        }
                                                                        board_clone_k[(k - dice.0) as usize].0 = 1;
                                                                        board_clone_k[(k - dice.0) as usize].1 = board_clone_k[(k - dice.0) as usize].1 + 1;
                                                                        three_move_set.insert(
                                                                            board_state { board: board_clone_k.clone(), 
                                                                                keep_count_1: keep_count_1_clone_k, 
                                                                                keep_count_2: keep_count_of_2_stones,
                                                                                hit_stones_1: hit_stones_1_clone, 
                                                                                hit_stones_2: hit_stones_2_clone_k,
                                                                                dice: dice.clone(),
                                                                                parent_index: -1
                                                                            }
                                                                        );
                                                                        // println!("k: {}", k);
                                                                        // for mut s in &three_move_set{
                                                                        //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                        // }
                                                                        //read_input();
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            println!("{}", three_move_set.len());
                            println!("{}", two_move_set.len());
                            println!("{}", one_move_set.len());
                            read_input();
                            if three_move_set.len() != 0{
                                println!("Three");
                                for mut s in set_board{
                                    display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                }
                                set_board = three_move_set.clone();
                            }else{
                                println!("we should be here");
                                if two_move_set.len() != 0{
                                    println!("Two");
                                    set_board = two_move_set.clone();
                                    for mut s in set_board{
                                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                    }
                                    
                                }else{
                                    if one_move_set.len() != 0{
                                        println!("One");
                                        set_board = one_move_set.clone();
                                        for mut s in set_board{
                                            display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                        }
                                        
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("Here in dice.0 == dice.1");
                    read_input();
                    let mut set_board: HashSet<board_state> = HashSet::new();
                    // Find an anchor 
                    // This is supposed to figure out the 4 possible moves
                    // if not 3 possible moves
                    // if not 2 possible moves
                    // if not 1  possible moves
                    let mut one_move_set: HashSet<board_state> = HashSet::new();
                    let mut two_move_set: HashSet<board_state> = HashSet::new();
                    let mut three_move_set: HashSet<board_state> = HashSet::new();
                    let mut four_move_set: HashSet<board_state> = HashSet::new();
                    // looping over the 
                    // I don't think we need these hashsets because we can
                    // just loop over stuff and be fine with stuff
                    
                    for i in (0..=23).rev(){
                        if(board[i as usize].0 == 1){
                            let mut board_clone_i: Vec<(i32, i32)> = board.clone();
                            let mut hit_stones_2_clone_i = hit_stones_2;    
                            let mut keep_count_1_clone_i = keep_count_of_1_stones;
                            // Going out
                            // we need to check if everything is in zone of going out
                            let mut all_in_zone = 1;
                            let mut did_move_in_i = 0;
                            for r in 6..=23{
                                if(board_clone_i[r].0 == 1){
                                    all_in_zone = 0;
                                }
                            }
                            if i - dice.0 < 0 && all_in_zone == 1{
                                keep_count_1_clone_i = keep_count_1_clone_i - 1;
                                board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                if board_clone_i[i as usize].1 == 0 {
                                    board_clone_i[i as usize].0 = 0
                                }
                                did_move_in_i = 1;
                                one_move_set.insert(
                                    board_state { board: board_clone_i.clone(), 
                                        keep_count_1: keep_count_1_clone_i, 
                                        keep_count_2: keep_count_of_2_stones,
                                         hit_stones_1: hit_stones_1, 
                                         hit_stones_2: hit_stones_2_clone_i,
                                        dice: dice.clone(),
                                        parent_index: -1
                                    }
                                );
                            // Moving on the board

                            } else{
                                if i - dice.0 >= 0{
                                    if !(board_clone_i[(i - dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 > 1){
                                        if(board_clone_i[(i- dice.0) as usize].0 == 2 && board_clone_i[(i - dice.0) as usize].1 == 1){
                                            hit_stones_2_clone_i = hit_stones_2_clone_i + 1;
                                            board_clone_i[(i - dice.0) as usize].0 = 0;
                                            board_clone_i[(i - dice.0) as usize].1 = 0;
                                        }
                                        board_clone_i[i as usize].1 = board_clone_i[i as usize].1 - 1;
                                        if(board_clone_i[i as usize].1 == 0){
                                            board_clone_i[i as usize].0 = 0;
                                        }
                                        board_clone_i[(i - dice.0) as usize].0 = 1;
                                        board_clone_i[(i - dice.0) as usize].1 = board_clone_i[(i - dice.0) as usize].1 + 1;

                                        did_move_in_i = 1;
                                        one_move_set.insert(
                                            board_state { board: board_clone_i.clone(), 
                                                keep_count_1: keep_count_1_clone_i, 
                                                keep_count_2: keep_count_of_2_stones,
                                                 hit_stones_1: hit_stones_1, 
                                                 hit_stones_2: hit_stones_2_clone_i,
                                                dice: dice.clone(),
                                                parent_index: -1
                                            }
                                        );
                                    }
                                }
                            }

                            // println!("i: {}", i);
                            // for mut s in &one_move_set{
                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                            // }
                            //read_input();
                            if did_move_in_i == 1{
                                for j in (0..=i).rev(){
                                    if(board_clone_i[j as usize].0 == 1){
                                        let mut board_clone_j: Vec<(i32, i32)> = board_clone_i.clone();
                                        let mut hit_stones_2_clone_j = hit_stones_2_clone_i;    
                                        let mut keep_count_1_clone_j = keep_count_1_clone_i;
                                        // Going out
                                        // we need to check if everything is in zone of going out
                                        let mut did_move_in_j = 0;
                                        let mut all_in_zone = 1;
                                        for r in 6..=23{
                                            if(board_clone_j[r].0 == 1){
                                                all_in_zone = 0;
                                            }
                                        }
                                        if j - dice.0 < 0 && all_in_zone == 1{
                                            keep_count_1_clone_j = keep_count_1_clone_j - 1;
                                            board_clone_j[i as usize].1 = board_clone_j[i as usize].1 - 1;
                                            if board_clone_j[i as usize].1 == 0 {
                                                board_clone_j[i as usize].0 = 0
                                            }
                                            did_move_in_j = 1;
                                            two_move_set.insert(
                                                board_state { board: board_clone_j.clone(), 
                                                    keep_count_1: keep_count_1_clone_j, 
                                                    keep_count_2: keep_count_of_2_stones,
                                                     hit_stones_1: hit_stones_1, 
                                                     hit_stones_2: hit_stones_2_clone_j,
                                                    dice: dice.clone(),
                                                    parent_index: -1
                                                }
                                            );
                                        // Moving on the board
                                        } else{
                                            if j - dice.0 >= 0{
                                                if !(board_clone_j[(j - dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 > 1){
                                                    if(board_clone_j[(j- dice.0) as usize].0 == 2 && board_clone_j[(j - dice.0) as usize].1 == 1){
                                                        hit_stones_2_clone_j = hit_stones_2_clone_j + 1;
                                                        board_clone_j[(j - dice.0) as usize].0 = 0;
                                                        board_clone_j[(j - dice.0) as usize].1 = 0;
                                                    }
                                                    board_clone_j[j as usize].1 = board_clone_j[j as usize].1 - 1;
                                                    if(board_clone_j[j as usize].1 == 0){
                                                        board_clone_j[j as usize].0 = 0;
                                                    }
                                                    board_clone_j[(j - dice.0) as usize].0 = 1;
                                                    board_clone_j[(j - dice.0) as usize].1 = board_clone_j[(j - dice.0) as usize].1 + 1;
                                                    did_move_in_j = 1;
                                                    two_move_set.insert(
                                                        board_state { board: board_clone_j.clone(), 
                                                            keep_count_1: keep_count_1_clone_j, 
                                                            keep_count_2: keep_count_of_2_stones,
                                                             hit_stones_1: hit_stones_1, 
                                                             hit_stones_2: hit_stones_2_clone_j,
                                                            dice: dice.clone(),
                                                            parent_index: -1
                                                        }
                                                    );
                                                    // println!("j: {}", j);
                                                    // for mut s in &two_move_set{
                                                    //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                    // }
                                                    //read_input();
                                                }
                                            }
                                        }
                                        if did_move_in_j == 1{
                                            for k in (0..=j).rev(){
                                                if(board_clone_j[k as usize].0 ==1){
                                                    let mut board_clone_k = board_clone_j.clone();
                                                    let mut hit_stones_2_clone_k = hit_stones_2_clone_j;    
                                                    let mut keep_count_1_clone_k = keep_count_1_clone_j;
                                                    // Going out
                                                    // we need to check if everything is in zone of going out
                                                    let mut did_move_in_k = 0;
                                                    let mut all_in_zone = 1;
                                                    for r in 6..=23{
                                                        if(board_clone_j[r].0 == 1){
                                                            all_in_zone = 0;
                                                        }
                                                    }
                                                    if k - dice.0 < 0 && all_in_zone == 1{
                                                        keep_count_1_clone_k = keep_count_1_clone_k - 1;
                                                        board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                        if board_clone_k[k as usize].1 == 0 {
                                                            board_clone_k[k as usize].0 = 0
                                                        }
                                                        did_move_in_k = 1;
                                                        three_move_set.insert(
                                                            board_state { board: board_clone_k.clone(), 
                                                                keep_count_1: keep_count_1_clone_k, 
                                                                keep_count_2: keep_count_of_2_stones,
                                                                 hit_stones_1: hit_stones_1, 
                                                                 hit_stones_2: hit_stones_2_clone_k,
                                                                dice: dice.clone(),
                                                                parent_index: -1
                                                            }
                                                        );
                                                    // Moving on the board
                                                    } else{
                                                        if k - dice.0 >= 0{
                                                            if !(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 > 1){
                                                                if(board_clone_k[(k - dice.0) as usize].0 == 2 && board_clone_k[(k - dice.0) as usize].1 == 1){
                                                                    hit_stones_2_clone_k = hit_stones_2_clone_k + 1;
                                                                    board_clone_k[(k - dice.0) as usize].0 = 0;
                                                                    board_clone_k[(k - dice.0) as usize].1 = 0;
                                                                }
                                                                board_clone_k[k as usize].1 = board_clone_k[k as usize].1 - 1;
                                                                if(board_clone_k[k as usize].1 == 0){
                                                                    board_clone_k[k as usize].0 = 0;
                                                                }
                                                                board_clone_k[(k - dice.0) as usize].0 = 1;
                                                                board_clone_k[(k - dice.0) as usize].1 = board_clone_k[(k - dice.0) as usize].1 + 1;
                                                                did_move_in_k = 1;
                                                                three_move_set.insert(
                                                                    board_state { board: board_clone_k.clone(), 
                                                                        keep_count_1: keep_count_1_clone_k, 
                                                                        keep_count_2: keep_count_of_2_stones,
                                                                         hit_stones_1: hit_stones_1, 
                                                                         hit_stones_2: hit_stones_2_clone_k,
                                                                        dice: dice.clone(),
                                                                        parent_index: -1
                                                                    }
                                                                );
                                                                // println!("k: {}", k);
                                                                // for mut s in &three_move_set{
                                                                //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                // }
                                                                //read_input();
                                                            }
                                                        }
                                                    }
                                                    if did_move_in_k == 1{
                                                        for l in (0..=k).rev(){
                                                            if(board_clone_k[l as usize].0 == 1){
                                                                let mut board_clone_l = board_clone_k.clone();
                                                                let mut hit_stones_2_clone_l = hit_stones_2_clone_k;    
                                                                let mut keep_count_1_clone_l = keep_count_1_clone_k;
                                                                // we need to check if everything is in zone of going out
                                                                let mut all_in_zone = 1;
                                                                for r in 6..=23{
                                                                    if(board_clone_k[r].0 == 1){
                                                                        all_in_zone = 0;
                                                                    }
                                                                }
                                                                if l - dice.0 < 0 && all_in_zone == 1{
                                                                    keep_count_1_clone_l = keep_count_1_clone_l - 1;
                                                                    board_clone_l[l as usize].1 = board_clone_l[l as usize].1 - 1;
                                                                    if board_clone_l[l as usize].1 == 0 {
                                                                        board_clone_l[l as usize].0 = 0
                                                                    }
                                                                    four_move_set.insert(
                                                                        board_state { board: board_clone_l.clone(), 
                                                                            keep_count_1: keep_count_1_clone_l, 
                                                                            keep_count_2: keep_count_of_2_stones,
                                                                             hit_stones_1: hit_stones_1, 
                                                                             hit_stones_2: hit_stones_2_clone_l,
                                                                            dice: dice.clone(),
                                                                            parent_index: -1
                                                                        }
                                                                    );
                                                                // Moving on the board
                                                                } else{
                                                                    if l - dice.0 >= 0{
                                                                        if !(board_clone_l[(l - dice.0) as usize].0 == 2 && board_clone_l[(l - dice.0) as usize].1 > 1){
                                                                            if(board_clone_l[(l - dice.0) as usize].0 == 2 && board_clone_l[(l - dice.0) as usize].1 == 1){
                                                                                hit_stones_2_clone_l = hit_stones_2_clone_l + 1;
                                                                                board_clone_l[(l - dice.0) as usize].0 = 0;
                                                                                board_clone_l[(l - dice.0) as usize].1 = 0;
                                                                            }
                                                                            board_clone_l[l as usize].1 = board_clone_l[l as usize].1 - 1;
                                                                            if(board_clone_l[l as usize].1 == 0){
                                                                                board_clone_l[l as usize].0 = 0;
                                                                            }
                                                                            board_clone_l[(l - dice.0) as usize].0 = 1;
                                                                            board_clone_l[(l - dice.0) as usize].1 = board_clone_l[(l - dice.0) as usize].1 + 1;
                                                                            four_move_set.insert(
                                                                                board_state { board: board_clone_l.clone(), 
                                                                                    keep_count_1: keep_count_1_clone_l, 
                                                                                    keep_count_2: keep_count_of_2_stones,
                                                                                    hit_stones_1: hit_stones_1, 
                                                                                    hit_stones_2: hit_stones_2_clone_l,
                                                                                    dice: dice.clone(),
                                                                                    parent_index: -1
                                                                                }
                                                                            );
                                                                            
                                                                            // println!("l: {}", l);
                                                                            // for mut s in &four_move_set{
                                                                            //     display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1.clone(), &mut s.keep_count_2.clone());
                                                                            // }
                                                                            // println!("LINE 3253");
                                                                            // read_input();
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    println!("{}", four_move_set.len());
                    println!("{}", three_move_set.len());
                    println!("{}", two_move_set.len());
                    println!("{}", one_move_set.len());
                    read_input();
                    if four_move_set.len() != 0{
                        set_board = four_move_set.clone();
                        for mut s in set_board{
                            println!("Four");
                            display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                        }
                    }else {
                        if three_move_set.len() != 0{
                            println!("Three");
                            for mut s in set_board{
                                display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                            }
                            set_board = three_move_set.clone();
                        }else{
                            if two_move_set.len() != 0{
                                println!("Two");
                                for mut s in set_board{
                                    display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                }
                                set_board = two_move_set.clone();
                            }else{
                                if one_move_set.len() != 0{
                                    println!("One");
                                    for mut s in set_board{
                                        display_board(&s.board, &s.hit_stones_1, &s.hit_stones_2, &mut s.keep_count_1, &mut s.keep_count_2);
                                    }
                                    set_board = one_move_set.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        
    }
}



// todo: figure out how to make the program in parallel with a mutex that keeps count of the file numbers
fn main() {
    // user::clear_terminal();
    // let mut game_mode: i32 = 0;
    // game_mode = user::user_menu();
    // user::clear_terminal();
    // println!("{}", game_mode);
    // _ = read_input();
    // user::clear_terminal();
    // // (user (1,2), destination_position(0...23), number_of_stones)
    // let mut history_of_a_game: Vec<(i32, i32, i32)> = vec![];
    // let mut history_of_dice: Vec<(i8, i8)> = vec![];
    // history_of_dice.push((4,3));
    // history_of_dice.push((4,5));
    // history_of_a_game.push((2, 4, 0));
    // history_of_a_game.push((2, 3, 0));
    // history_of_a_game.push((1, 19, 23));
    // history_of_a_game.push((1, 7, 12));
    // history_of_a_game.push((2, 19, 18));
    // history_of_a_game.push((2, 23, 18));
    // println!("history_of_a_game: {:?}", history_of_a_game);
    // let mut history_of_games: Vec<(Vec<(i32, i32, i32)>, Vec<(i32, i32)>)> = vec![];
    // //history_of_games.push((history_of_a_game, history_of_dice));
    // println!("history_of_games: {:?}", history_of_games);
    // // let mut tmp_vec: Vec<i32> = vec![];
    // // tmp_vec.push(1);
    // // tmp_vec.push(10);
    // // println!("tmp_vec: {:?}", tmp_vec);
    // // mut_vec(&mut tmp_vec);
    // // println!("tmp_vec: {:?}", tmp_vec);
    // let mut rng = rand::thread_rng();
    // // Generate a random number in the range [0, 7)
    // let n1: i32 = rng.gen_range(1..7);
    // println!("Random number between 1 and inclusive 7: {}", n1);

    // =============================================================
    // =========================TEST================================
    // =============================================================

    // let mut board: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
    //                                     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    //                                     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    //                                     (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];

    // let mut board2: Vec<(i32, i32)> = vec![(0,0), (2,1), (2,1), (0,0), (0,0), (1,5), 
    //                                     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    //                                     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    //                                     (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];

    
    // let mut hit_stones_1 : i32 = 0;
    // let mut hit_stones_2 : i32 = 0;
    // let mut keep_count_of_1_stones: i32 = 15;
    // let mut keep_count_of_2_stones: i32 = 15;
    // generate_boards(&board, 1, keep_count_of_1_stones, keep_count_of_2_stones, hit_stones_1, hit_stones_2);


    // hit_stones test
    let mut board_t: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
    (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    (2,3), (0,0), (2,1), (0,0), (2,1), (0,0)];
    let mut hit_stones_1_t : i32 = 2;
    let mut hit_stones_2_t : i32 = 0;
    let mut keep_count_of_1_stones_t: i32 = 15;
    let mut keep_count_of_2_stones_t: i32 = 15;
    generate_boards(&board_t, 1, keep_count_of_1_stones_t, keep_count_of_2_stones_t, hit_stones_1_t, hit_stones_2_t);


    // hit_stones test
    // let mut board_h: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
    // (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    // (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    // (2,3), (0,0), (2,1), (1,1), (2,1), (0,0)];
    // let mut hit_stones_1_t : i32 = 1;
    // let mut hit_stones_2_t : i32 = 0;
    // let mut keep_count_of_1_stones_t: i32 = 15;
    // let mut keep_count_of_2_stones_t: i32 = 15;
    // generate_boards(&board_h, 1, keep_count_of_1_stones_t, keep_count_of_2_stones_t, hit_stones_1_t, hit_stones_2_t);

    // hit_stones test TURN 2
    // let mut board_t: Vec<(i32, i32)> = vec![(0,0), (1,1), (0,0), (1,1), (0,0), (1,3), 
    // (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    // (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    // (2,3), (0,0), (2,1), (0,0), (2,1), (0,0)];
    // let mut hit_stones_1_t : i32 = 0;
    // let mut hit_stones_2_t : i32 = 2;
    // let mut keep_count_of_1_stones_t: i32 = 15;
    // let mut keep_count_of_2_stones_t: i32 = 15;
    // generate_boards(&board_t, 2, keep_count_of_1_stones_t, keep_count_of_2_stones_t, hit_stones_1_t, hit_stones_2_t);

    // hit_stones test 2 TURN 2
    // let mut board_t: Vec<(i32, i32)> = vec![(0,0), (1,1), (2,1), (1,1), (0,0), (1,3), 
    // (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    // (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    // (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
    // let mut hit_stones_1_t : i32 = 0;
    // let mut hit_stones_2_t : i32 = 1;
    // let mut keep_count_of_1_stones_t: i32 = 15;
    // let mut keep_count_of_2_stones_t: i32 = 15;
    // generate_boards(&board_t, 2, keep_count_of_1_stones_t, keep_count_of_2_stones_t, hit_stones_1_t, hit_stones_2_t);




    // =============================================================
    // =====================END TEST GENERATE BOARD=================
    // =============================================================



    // calc_stats_board(&board, keep_count_of_1_stones, keep_count_of_2_stones, hit_stones_1, hit_stones_2);
    // calc_stats_board(&board2, keep_count_of_1_stones, keep_count_of_2_stones, hit_stones_1, hit_stones_2);

    // first element is the current board, with its current dice, and moves it can do
    // then whoever turn is, the possible boards for that player
    // we connect each others with children pointing to the parent 
    // we bottom up so the first element can have a decsion
    //let mut levels: Vec<> = vec![]

    
    
    // play(1, &mut history_of_games, game_mode);
    // let mut board: Vec<(i32, i32)> = vec![(1,2), (1,2), (1,5), (1,5), (1,1), (0,0),
    //                                     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    //                                     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    //                                     (0,0), (2,5), (2,3), (2,5), (2,2), (0,0)];
    //display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    
    // let mut board: Vec<(i32, i32)> = vec![(2,2), (1,1), (1,1), (1,1), (1,1), (1,1),
    //                                     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5),
    //                                     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0),
    //                                     (2,3), (2,1), (0,0), (0,0), (1,1), (1,1)];
    
    // hit_stones_2 = 1;
    // display_board(&board, &hit_stones_1, &hit_stones_2);

    // history_of_a_game.push((1, 5, 7));
    // history_of_a_game.push((1, 3, 7));
    // history_of_a_game.clear();


    // for game_move in &history_of_a_game{
    //     println!("{}, {}, {}",game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2);
    //     display_board(&board, &hit_stones_1, &hit_stones_2);
    // }
    // history_of_a_game.push((2, 4, -1));
    // history_of_a_game.push((2, 19, 18));
    // for game_move in &history_of_a_game{
    //     println!("{}, {}, {}", game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2);
    //     display_board(&board, &hit_stones_1, &hit_stones_2);
    // }
    // ***************************************************************************************************************
    // println!("Hello");
    // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    // history_of_a_game.push((2, -2, 20));
    // history_of_a_game.push((2, -2, 19));

    // history_of_a_game.push((1, -2, 0));
    // history_of_a_game.push((1, -2, 1));

    // history_of_a_game.push((2, -2, 20));
    // history_of_a_game.push((2, -2, 22));

    // for game_move in &history_of_a_game{
    //     //println!("{}, {}, {}", game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    //     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    // }
    // ***************************************************************************************************************
    // let number = read_input();
    // println!("You entered: {}", number);
    // let start = Instant::now();
    // // play(100000, &mut history_of_games);
    
    // //let mut num = COUNTER.lock().unwrap();
    // //*num = read_input();
    // let mut win1 = 0;
    // let mut win2 = 0;

    // _ = read_file_game_history(&mut history_of_games);
    // println!("salam {}", history_of_games.len());
    // for i in 50000..60000{
    //     //println!("FUCK: {} ", history_of_games[i].0.len());
    //     for j in 0..(history_of_games[i].0.len()){
    //         //println!("i: {}", i);
    //         //println!("j: {}", j);
    //         //println!("game_move: {}, {}, {}", history_of_games[i].0[j].0, history_of_games[i].0[j].1, history_of_games[i].0[j].2);
    //         update_board(&mut board, &history_of_games[i].0[j], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    //         //display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    //         //write_to_file_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);

    //     }
    //     if keep_count_of_1_stones > keep_count_of_2_stones{
    //         win2 += 1;
    //     } else {
    //         win1 += 1;
    //     }
    //     board = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
    //     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    //     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    //     (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
    //     hit_stones_1 = 0;
    //     hit_stones_2 = 0;
    //     keep_count_of_1_stones = 15;         
    //     keep_count_of_2_stones = 15;
    // }
    // let duration = start.elapsed();
    // println!("Time taken by some_function: {:?}", duration);
    // println!("{}, {}", win1, win2);

    //write_to_file_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);

    // for i in 0..1{
    //     for j in 0..(history_of_games[i].1.len()){
    //         println!("j: {}", j);
    //         println!("dice: {}, {}", history_of_games[i].1[j].0, history_of_games[i].1[j].1);
    //         // update_board(&mut board, &history_of_games[i].0[j], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    //         // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones)
    //     }
    //     board = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
    //     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
    //     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
    //     (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
    // }
}



// The things that need a hyperparameter to start with
// how offensive you suold be, how defencive, how forwarding
// learn how to parallalize this shit
// we have to make a decision function (A mixed of data science and in depth computation)

