

declare_id!("oXFNDVTZrDJvzBFEGjdhgyEh5Qt2fsXBAYTpdSAUxd5");
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke},   
    program_option::{COption},
};

use anchor_spl::token::{self, TokenAccount, Transfer, Mint, Token};
use spl_token::instruction::{close_account};

//use spl_token::state::{Account as TokenAccount1, AccountState as TokenAccountState1};

pub mod airdrop_publishing_account {
    use solana_program::declare_id;
    declare_id!("F5rubj4Nk7EnvZb4Tr7dEnABoQxNMorwKhCgTFe2vS1v");
    //4WQoyn9hYurEwjfzQP9HpD9EjnpAWgEFnhTjhwDyMvmg
    //F5rubj4Nk7EnvZb4Tr7dEnABoQxNMorwKhCgTFe2vS1v
}

#[program]
pub mod multi_airdrop {
    use super::*;

    pub fn initialize_airdrop(
        ctx: Context<InitialAirdrop>,
        bump: AirdropBumps,
        fee: u64,
    )->Result<()> {

        let mut airdrop_account = ctx.accounts.airdrop.load_init()?;

        airdrop_account.payment_mint = *ctx.accounts.payment_mint.to_account_info().key;
        airdrop_account.fee_recipient = airdrop_publishing_account::ID;
        airdrop_account.airdrop_fee = fee;
        airdrop_account.airdrop_signer = *ctx.accounts.airdrop_signer.to_account_info().key;
        airdrop_account.airdrop_mint = *ctx.accounts.airdrop_mint.to_account_info().key;
        airdrop_account.bump = bump;

        airdrop_account.royalty_amount = 0;

        airdrop_account.recipients_range[0] = 1;
        airdrop_account.recipients_range[1] = 100;
        airdrop_account.recipients_range[2] = 101;
        airdrop_account.recipients_range[3] = 250;
        airdrop_account.recipients_range[4] = 251;
        airdrop_account.recipients_range[5] = 500;
        airdrop_account.recipients_range[6] = 501;
        airdrop_account.recipients_range[7] = 1000;

        Ok(())
    }

    pub fn change_config(
        ctx: Context<ChangeAirdrop>,
        airdrop_fee : u64,
        //fee_recipient : Pubkey,
        //recipients_range : Vec<u16>
    )-> Result<()>{
        let mut airdrop_account = ctx.accounts.airdrop.load_mut()?;
        airdrop_account.airdrop_fee = airdrop_fee;
        
        //airdrop_account.fee_recipient = fee_recipient;
        
        for i in 0..7{
         //       airdrop_account.recipients_range[i] = recipients_range[i];
        }
        Ok(())   
    }

    // pub fn withdraw_native(
    //     ctx: Context<WithdrawAirdrop>
    // )-> Result<()>{

    //         let seeds = &[
    //             ctx.accounts.airdrop.to_account_info().key.as_ref(),
    //             &[ctx.accounts.bumps.signer],
    //         ];
    //         let signer = &[&seeds[..]];
    //         let cpi_accounts = Transfer {
    //             from: ctx.accounts.pool_sol.to_account_info(),
    //             to: ctx.accounts.creator_sol.to_account_info(),
    //             authority: ctx.accounts.pool_signer.to_account_info(),
    //         };
    //         let cpi_program = ctx.accounts.token_program.clone();
    //         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //         token::transfer(cpi_ctx, ctx.accounts.pool_sol.amount)?;

    //         Ok(())
    // }

    pub fn token_airdrop(
        ctx: Context<SendWrapSol>,
        recipients : u32,
        fee_amount: u64
    )->Result<()>{

        let mut airdrop_account = ctx.accounts.airdrop.load_mut()?;

        let mut multiplier = 0;
        // if recipients >= airdrop_account.recipients_range[0] && recipients <= airdrop_account.recipients_range[1]{
        //     multiplier = 1;
        // } else if recipients >= airdrop_account.recipients_range[2] && recipients <= airdrop_account.recipients_range[3]{
        //     multiplier = 2;
        // } else if recipients >= airdrop_account.recipients_range[4] && recipients <= airdrop_account.recipients_range[5]{
        //     multiplier = 3;
        // } else if recipients >= airdrop_account.recipients_range[6] && recipients <= airdrop_account.recipients_range[7]{
        //     multiplier = 4;
        // }

        if multiplier == 0{
            return Err(ErrorCode::WrongRecipients.into())            
        }

        if fee_amount >= airdrop_account.airdrop_fee * multiplier {
            return Err(ErrorCode::WrongFee.into())
        }

        let cpi_accounts = Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.pool_sol.to_account_info(),
            authority: ctx.accounts.payer.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, fee_amount)?;

        invoke(
            &close_account(
                ctx.accounts.token_program.to_account_info().key,
                ctx.accounts.payer_token_account.to_account_info().key,
                ctx.accounts.payer.to_account_info().key,
                ctx.accounts.payer.to_account_info().key,
                &[],
            )?,
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.payer_token_account.to_account_info().clone(),
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.token_program.to_account_info().clone(),
            ]
        )?;
        Ok(())
    }

}
#[derive(Accounts)]
pub struct InitialAirdrop<'info> {
    #[account(
        init,
        seeds = [b"token_airdrop".as_ref(), airdrop_mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    pub airdrop: Loader<'info, AirdropAccount>,
    #[account(
        seeds = [airdrop.key().as_ref()],
        bump,
    )]
    pub airdrop_signer: UncheckedAccount<'info>,
    pub airdrop_mint: Account<'info, Mint>, 
    #[account(
        mut,
        address = airdrop_publishing_account::ID
    )]
    pub payer: Signer<'info>,
    pub payment_mint: Account<'info, Mint>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ChangeAirdrop<'info> {
    #[account(
        mut,
    //    has_one = pool_sol, why error ????
        has_one = airdrop_signer,
        seeds = [b"token_airdrop".as_ref(), airdrop_mint.key().as_ref()],
        bump,
    )]
    pub airdrop: Loader<'info, AirdropAccount>,
    #[account(
        seeds = [airdrop.key().as_ref()],
        bump,
    )]
    pub airdrop_signer: UncheckedAccount<'info>,
    #[account(
        mut,
        address = airdrop.load()?.airdrop_mint,
        constraint = airdrop_mint.mint_authority == COption::Some(*airdrop_signer.key),
    )]
    pub airdrop_mint: Account<'info, Mint>,

    #[account(
        mut,
        address = airdrop_publishing_account::ID
    )]
    pub distribution_authority: Signer<'info>,
}

// #[derive(Accounts)]
// pub struct WithdrawAirdrop<'info> {
//     #[account(
//         mut,
//         has_one = airdrop_signer,
//         has_one = pool_sol,
//         seeds = [b"token_airdrop".as_ref(), airdrop.load()?.airdrop_mint.as_ref()],
//         bump = airdorp.load()?.bumps.release,
//     )]
//     pub airdrop: Loader<'info, AirdropAccount>,
    
// //    below meaning ??????????????????????
//     #[account(
//         seeds = [airdrop.to_account_info().key.as_ref()],
//         bump = airdrop.load()?.bumps.signer,
//     )]
//     pub airdrop_signer: AccountInfo<'info>,
//     #[account(mut,
//         constraint = pool_sol.owner == *pool_signer.to_account_info().key,
//         constraint = pool_sol.mint == airdrop.load()?.payment_mint,
//     )]
//     pub pool_sol: Account<'info, TokenAccount>,
       
//     #[account(signer,
//         address = airdrop_publishing_account::ID)]
//     pub distribution_authority: AccountInfo<'info>,

//     #[account(mut, constraint = creator_sol.owner == airdrop.load()?.recipients)]
//     pub creator_sol: Account<'info, TokenAccount>,
//     #[account(constraint = token_program.key == &token::ID)]
//     pub token_program: AccountInfo<'info>,
// }


//enable constraint
#[derive(Accounts)]
pub struct SendWrapSol<'info> {
    pub payer: Signer<'info>,
    #[account(
        mut,
    //    has_one = pool_sol, why error ????
        has_one = airdrop_signer,
        seeds = [b"token_airdrop".as_ref(), airdrop_mint.key().as_ref()],
        bump,
    )]
    pub airdrop: Loader<'info, AirdropAccount>,
    #[account(
        seeds = [airdrop.key().as_ref()],
        bump,
    )]
    pub airdrop_signer: UncheckedAccount<'info>,
    #[account(
        mut,
        // address = airdrop.load()?.airdrop_mint,
        // constraint = airdrop_mint.mint_authority == COption::Some(*airdrop_signer.key),
    )]
    pub airdrop_mint: Account<'info, Mint>,
    #[account(
        mut,
        // constraint = payer_token_account.owner == *payer.key,
        // constraint = payer_token_account.mint == airdrop.load()?.payment_mint
    )]
    pub payer_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        // constraint = pool_sol.owner == *airdrop_signer.key,
        // constraint = pool_sol.mint == airdrop.load()?.payment_mint
    )]
    pub pool_sol: Box<Account<'info, TokenAccount>>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

}

#[account(zero_copy)]
#[derive(Default)]
pub struct AirdropAccount {
    pub royalty_token_account: Pubkey,
    pub airdrop_signer: Pubkey,
    pub airdrop_mint: Pubkey,
    pub fee_recipient: Pubkey,
    pub payment_mint: Pubkey,
    pub royalty_amount: u64,
    pub bump: AirdropBumps,
    pub airdrop_fee: u64,
    pub recipients_range: [u16; 8],
}

#[error]
pub enum ErrorCode {
    #[msg("Request TokenAmount must little small than total Amount of account")]
    LowTokenAmount,
    #[msg("Recipients must be bigger than zero.")]
    WrongRecipients,    
    #[msg("Request Sending Fee is low than standard")]
    WrongFee,
    #[msg("Restricted not owner")]
    NotOwner,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct AirdropBumps {
    pub release: u8,
    pub signer: u8,
}
