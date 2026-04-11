use anchor_lang::prelude::*;

declare_id!("Cpp3D9s8qjJ3vN3GYfLAuF2hXrrJvfVU8RQDcXfWpyQG");

#[program]
pub mod drishti_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
