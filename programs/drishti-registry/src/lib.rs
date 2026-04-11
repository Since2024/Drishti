use anchor_lang::prelude::*;

declare_id!("BGWW5iqBEDgv5xHzvqPWfvoWSVqBmydbc6xhWRX2jArn");

#[program]
pub mod drishti_registry {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
