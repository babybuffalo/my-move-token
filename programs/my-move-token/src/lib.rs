use anchor_lang::prelude::*;

declare_id!("6ySFsZ7tQ66zmLpuhfri8FhMzFDHNQeGWbGwsAuzJst4");

#[program]
pub mod my_move_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
