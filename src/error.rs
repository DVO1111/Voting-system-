// Custom error types for the voting system

#[derive(Debug)]
pub enum VotingError {
    InvalidVote,
    VoterNotRegistered,
    VotingClosed,
    UnexpectedError,
}

impl std::fmt::Display for VotingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for VotingError {}