use anchor_lang::prelude::*;

declare_id!("9ZWXmZErPWxScbjBFvNKWiWRTHDxrcqAAK96a1KzM2Gi");

#[program]
pub mod drishti_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
