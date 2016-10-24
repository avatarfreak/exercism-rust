// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let which_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self::new(self.x, self.y, which_direction)
    }
    pub fn turn_left(self) -> Self {
        let which_direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        Self::new(self.x, self.y, which_direction)
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::South => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::West => (self.x - 1, self.y),
        };
        Self::new(x, y, self.direction)
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.to_uppercase()
            .trim()
            .chars()
            .fold(self, |mut robot, command| {
                robot = match command {
                    'A' => robot.advance(),
                    'L' => robot.turn_left(),
                    'R' => robot.turn_right(),
                    _ => robot,
                };
                robot
            })
    }



    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
