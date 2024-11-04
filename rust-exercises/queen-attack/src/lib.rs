#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    // Creates a new ChessPosition only if the coordinates are within the 8x8 board
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition(rank, file))
        } else {
            None
        }
    }
}

impl Queen {
    // Creates a new Queen with a given ChessPosition
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    // Checks if the Queen can attack another Queen based on row, column, or diagonal
    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition(row1, col1) = self.position;
        let ChessPosition(row2, col2) = other.position;

        // Check if on the same row, column, or diagonal
        row1 == row2 || col1 == col2 || (row1 - row2).abs() == (col1 - col2).abs()
    }
}
