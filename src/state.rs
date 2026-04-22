// src/state.rs

// Solana account state structures for voter registration, vote storage, and verification data

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VoterRegistration {
    pub voter_id: String,
    pub has_voted: bool,
    pub registration_time: i64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VoteStorage {
    pub vote_id: String,
    pub voter_id: String,
    pub candidate_id: String,
    pub vote_time: i64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VerificationData {
    pub vote_id: String,
    pub voter_signature: Vec<u8>,
    pub // any additional verification fields
}