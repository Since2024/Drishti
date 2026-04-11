use anchor_lang::prelude::*;

declare_id!("B7iyY173mcm4aVKAkKWLkZ8ngnKCdZ5cNBQExVsvchP5");

#[program]
pub mod drishti_registry {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
