 #[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.file || self.0.file == other.0.file || self.is_on_same_diagonal(other.0)
    }

    fn is_on_same_diagonal(&self, other: ChessPosition) -> bool {
        (self.0.rank - other.rank).abs() == (self.0.file - other.file).abs()
    }
}

#[derive(Debug)]
pub struct Pawn(ChessPosition);

impl Pawn {
    pub fn new(position: ChessPosition) -> Self {
        Pawn(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let pawn_rank = self.0.rank;
        let pawn_file = self.0.file;
        let queen_rank = other.0.rank;
        let queen_file = other.0.file;

        // Check if the queen is in the same file and one rank ahead or behind the pawn
        (pawn_file == queen_file) && ((pawn_rank - queen_rank).abs() == 1)
    }
}
