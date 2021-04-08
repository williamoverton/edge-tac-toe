use rand::Rng;

#[derive(Debug)]
pub struct Board {
    cells: Vec<BoardCell>,
    our_cell_value: bool,
}

impl Board {
    
    pub fn new() -> Self {
        let mut cells: Vec<BoardCell> = Vec::new();
        for _ in 0..9 {
            cells.push(BoardCell::new());
        }

        Board {
            cells: cells,
            our_cell_value: true,
        }
    }

    pub fn print_board(&self) {
        println!(
            "{}",
            self.get_board_string());
    }

    pub fn get_board_string(&self) -> String {
        let mut body = String::new();

        body.push_str(format!(
            "{}{}{}{}{}{}{}{}{}",
            self.cells[0].get_char(),
            self.cells[1].get_char(),
            self.cells[2].get_char(),
            self.cells[3].get_char(),
            self.cells[4].get_char(),
            self.cells[5].get_char(),
            self.cells[6].get_char(),
            self.cells[7].get_char(),
            self.cells[8].get_char(),
        ).as_str());

        body
    }

    pub fn read_from_string(&mut self, input: &str) -> Result<(), &str> {
        if input.len() != 9 {
            return Err("Invalid board string, must be 9 chars long");
        }

        for i in 0..input.len() {
            let ic = char::from(input.as_bytes()[i]);

            if ic == '-' {
                continue;
            };

            self.cells[i].set_owner(ic == 'X').unwrap();
        }

        Ok(())
    }

    pub fn pick(&mut self) -> Result<ActionStatus, ActionError> {
        match self.get_board_status() {
            Ok(status) => {
                if status != ActionStatus::Picked {
                    return Ok(status);
                }
            }
            Err(_err) => {}
        };

        match self.make_crucial_move() {
            Ok(status) => Ok(status),
            Err(why) => {
                println!("Weird!");
                println!("{:?}", why);
                Err(ActionError::NoFreeSpaces)
            }
        }
    }

    pub fn is_cells_available(&self) -> bool {
        self.cells.iter().any(|cell| cell.is_owned)
    }

    fn get_lines(&self) -> [[usize; 3]; 8] {
        [
            //hz
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            //vert
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            //diag
            [0, 4, 8],
            [2, 4, 6],
        ]
    }

    fn make_crucial_move(&mut self) -> Result<ActionStatus, ActionChoiceError> {
        let lines = self.get_lines();

        for i in 0..lines.len() {
            if self.check_line_for_possible_outcome(lines[i]) != LineOutcome::Nothing {
                self.place_in_line(lines[i]);
                return self.get_board_status();
            };
        }

        return self.make_random_choice();
    }

    fn get_board_status(&self) -> Result<ActionStatus, ActionChoiceError> {
        let lines = self.get_lines();

        for i in 0..lines.len() {
            let mut player_count = 0;
            let mut our_count = 0;

            for &cell_id in lines[i].iter() {
                if self.cells[cell_id as usize].is_owned {
                    if self.cells[cell_id as usize].owner == self.our_cell_value {
                        our_count += 1;
                    } else {
                        player_count += 1;
                    }
                }
            }

            if player_count == 3 {
                return Ok(ActionStatus::PlayerWon);
            }else if our_count == 3 {
                return Ok(ActionStatus::AIWon);
            }
        };

        if !self.is_cells_available() {
            return Ok(ActionStatus::Draw);
        }

        Ok(ActionStatus::Picked)
    }

    fn make_random_choice(&mut self) -> Result<ActionStatus, ActionChoiceError> {
        let mut free_cells = Vec::new();

        for i in 0..self.cells.len() {
            if !self.cells[i].is_owned {
                free_cells.push(i);
            }
        }

        if free_cells.len() == 0 {
            return Err(ActionChoiceError::NoChoiceMade);
        }

        let roll = rand::thread_rng().gen_range(0..free_cells.len());

        self.cells[free_cells[roll]]
            .set_owner(self.our_cell_value)
            .unwrap();

        self.get_board_status()
    }

    fn place_in_line(&mut self, cell_ids: [usize; 3]) {
        for &i in &cell_ids {
            if !self.cells[i].is_owned {
                self.cells[i].set_owner(self.our_cell_value).unwrap();
                break;
            }
        }
    }

    fn check_line_for_possible_outcome(&self, cell_ids: [usize; 3]) -> LineOutcome {
        let mut owned_count = 0;
        let mut we_own = 0;
        let mut player_owns = 0;

        for &i in &cell_ids {
            if !self.cells[i].is_owned {
                continue;
            }

            owned_count += 1;

            if self.cells[i].owner == self.our_cell_value {
                we_own += 1;
            } else {
                player_owns += 1;
            }
        }

        if owned_count != 2 {
            return LineOutcome::Nothing;
        }

        if we_own == 2 {
            return LineOutcome::CanWin;
        }

        if player_owns == 2 {
            return LineOutcome::PlayerCouldWin;
        }

        LineOutcome::Nothing
    }
}

#[derive(Debug, PartialEq)]
enum LineOutcome {
    CanWin,
    PlayerCouldWin,
    Nothing,
}

#[derive(Debug)]
pub enum ActionError {
    NoFreeSpaces,
}

#[derive(Debug)]
pub enum ActionChoiceError {
    NoChoiceMade,
}

#[derive(Debug, PartialEq)]
pub enum ActionStatus {
    Picked,
    AIWon,
    PlayerWon,
    Draw,
}

#[derive(Debug)]
pub struct BoardCell {
    pub owner: bool,
    pub is_owned: bool,
}

impl BoardCell {
    pub fn new() -> Self {
        BoardCell {
            owner: false,
            is_owned: false,
        }
    }

    pub fn set_owner(&mut self, owner: bool) -> Result<(), &str> {
        if self.is_owned {
            return Err("This cell is already owned!");
        }

        self.is_owned = true;
        self.owner = owner;

        Ok(())
    }

    pub fn get_char(&self) -> &str {
        if !self.is_owned {
            return "-";
        };

        match self.owner {
            true => "X",
            false => "O",
        }
    }
}
