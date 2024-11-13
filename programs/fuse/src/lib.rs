use anchor_lang::prelude::*;

declare_id!("AuccVXwMAEKwqoqT6qztCYxpg1W4RY3rBJdxUxLKSq7z");

#[program]
pub mod fuse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
