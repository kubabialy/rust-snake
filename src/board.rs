pub struct Board {
    width: u32,
    height: u32
}

// Board constructor
pub fn create_board(width: u32, height: u32) -> Board {
    return Board {
        width: width,
        height: height
    };
}

impl Board {
    pub fn print_self(&self) {
        for line in 0..self.height {
            let write_line: String;

            if line == 0 || line == self.height - 1 {
                write_line = std::iter::repeat("X").take(self.width as usize).collect::<String>();
            } else {
                write_line = format!("X{}X", std::iter::repeat(" ").take((self.width - 2) as usize).collect::<String>());
            }

            println!("{}", write_line);
        }
    }
}
