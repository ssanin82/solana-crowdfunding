pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2K1ddAKsCZKq9eXqL6xu64wtBJPCa4bzRQr547aw5wCo");

#[program]
pub mod solana_crowdfunding {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        initialize::handler(ctx)
    }
}
