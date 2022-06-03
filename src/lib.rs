use anchor_lang::prelude::*;

/*use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};*/
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS");

//declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_solana_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64,mint_seed: Vec<u8>,mint_bump: u8) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

/*        let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts =puppet::cpi::accounts::SetData {
            puppet: ctx.accounts.my_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, data);
*/
        Ok(())
    }
/*    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }*/


    pub fn airdrop(ctx: Context<Airdrop>, mint_seed: Vec<u8>,mint_bump: u8) -> Result<()> {

        Ok(())
    }
}




#[derive(Accounts)]
#[instruction(bump: u8,mint_seed: Vec<u8>)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,

/*    #[account(
    init,
    seeds = [b"seed".as_ref()],
    bump ,
    payer = user,space = 8 + 8
    )]
    pub my_pda_account: Account<'info, MyAccount>,*/


/*    #[account(
    init_if_needed,
    payer = user,
    seeds = [&mint_seed],
    bump ,
    mint::decimals = 9,
    mint::authority = mint
    )]*/
   // pub mint: Account<'info, Mint>,


    //pub puppet_program: Program<'info, Puppet>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}













#[derive(Accounts)]
pub struct Airdrop<'info> {
    pub mint: Account<'info, Mint>,
    pub destination: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
   // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
  //  pub associated_token_program: Program<'info, AssociatedToken>,
  //  pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct MyAccount {
    pub data: u64,

}