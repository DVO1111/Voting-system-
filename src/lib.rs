// src/lib.rs

use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod voting_system {
    use super::*;

    pub fn register_voter(ctx: Context<RegisterVoter>, phone_number: String, biometric_hash: String) -> ProgramResult {
        let voter = &mut ctx.accounts.voter;
        voter.phone_number = phone_number;
        voter.biometric_hash = biometric_hash;
        voter.has_voted = false;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, voter_id: Pubkey) -> ProgramResult {
        let voter = &mut ctx.accounts.voter;
        require!(!voter.has_voted, ErrorCode::AlreadyVoted);
        voter.has_voted = true;
        Ok(())
    }
}

#[account]
pub struct Voter {
    pub phone_number: String,
    pub biometric_hash: String,
    pub has_voted: bool,
}

#[derive(Accounts)]
pub struct RegisterVoter<'info> {
    #[account(init)]
    pub voter: &mut Account<'info, Voter>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Account<'info, Voter>,
    pub signer: Signer<'info>,
}

#[error]
pub enum ErrorCode {
    #[msg("The voter has already cast their vote.")]
    AlreadyVoted,
}

