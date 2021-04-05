mod board;

use std::io;

fn main() {
    let (width, height) = fetch_dimesnions();
    let board = board::create_board(width, height);
    board.print_self();
}

/**  
 * Grabs dimensions of the board from the user
 * and then tries to convert that into u32 types
 * required by Board struct
 */
fn fetch_dimesnions() -> (u32, u32) {
    println!("Provide width of the board");

    // Preparing variables to get init parameters from user
    // and to be casted into int later on
    let mut width = String::new();
    let mut height = String::new();

    io::stdin()
        .read_line(&mut width)
        .expect("Width is required");

    println!("Provide height of the board");

    io::stdin()
        .read_line(&mut height)
        .expect("Height is required");

    let width: u32 = width.trim()
        .parse()
        .expect("Provided width is not a number");

    if width < 1 { 
        panic!("Width must be a positive number");
    }

    let height: u32 = height.trim()
        .parse()
        .expect("Provided height is not a number");

    if height < 1 {
        panic!("Height must be a positive number");
    }

    return (width, height);
}
