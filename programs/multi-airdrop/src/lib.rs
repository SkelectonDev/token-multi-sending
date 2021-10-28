

declare_id!("6xi2VWSkxXcxoXm8snHunB3CyWfxLKrUPm3nC4EXRtk9");
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke},   
};

use anchor_spl::token::{self, TokenAccount, Transfer, Mint, Token};
use spl_token::instruction::{close_account};

//use spl_token::state::{Account as TokenAccount1, AccountState as TokenAccountState1};

pub mod airdrop_publishing_account {
    use solana_program::declare_id;
    declare_id!("3vios4fgwoDcrq1Vys2g9t5gcQMcDAWg3PXDLBvKRGmZ");
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
        airdrop_account.authority = *ctx.accounts.authority.to_account_info().key;
        airdrop_account.airdrop_fee = fee;
        airdrop_account.airdrop_signer = *ctx.accounts.airdrop_signer.to_account_info().key;
        airdrop_account.airdrop_mint = *ctx.accounts.airdrop_mint.to_account_info().key;
        airdrop_account.bump = bump;

        airdrop_account.royalty_amount = 0;
        //airdrop_account.recipientsRange[0] = 1;
        //airdrop_account.recipientsRange[1] = 100;
        Ok(())
    }
    pub fn send_token(
        ctx: Context<SendToken>,
        multi_account: Vec<TransactionData>        
    )->Result<()> {
        //let mut accounter: TokenAccount;
        // let accounter = TokenAccount::try_deserialize(buf);
        // let user = accounter.to_account_info();
        // (account,_a) = spl_token::state::Account::unpack(buf);
    //    let tmp_account = &mut ctx.accounts.taker_token_account;
    //    tmp_account.owner = multi_accounts[0].owner;

        // let account = TokenAccount1 {
        //     state: TokenAccountState1::Initialized,
        //     mint: multi_accounts[0].mint,
        //     owner: multi_accounts[0].owner,
        //     amount : 1multi_accounts[0].amount,
        //     ..Default::default()
        // };

        let multiaccount = &mut ctx.accounts.multi_sender;
        multiaccount.owner = multi_account[0].owner;
        multiaccount.mint = multi_account[0].mint;
        multiaccount.amount = multi_account[0].amount;
      
        // let mut multiaccount = &mut ctx.accounts.taker_token_account;
        // multiaccount.owner = multi_account[0].owner;
        // multiaccount.mint = multi_account[0].mint;
        // multiaccount.amount = multi_account[0].amount;
            
//        ctx.accounts.taker_token_account.
        
        //Airdrop Tokens to other receiver token accounts
        let cpi_accounts = Transfer {
            from: ctx.accounts.creator_token_account.to_account_info(),
            //to: ctx.accounts.taker_token_account.to_account_info(),
            to: ctx.accounts.multi_sender.to_account_info(),
            authority: ctx.accounts.distribution_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 20)?;

        Ok(())
    }
    pub fn send_wrap_sol(
        ctx: Context<SendWrapSol>,
        amount: u64
    )->Result<()>{

//        let mut multiplier:i8 = 0;
        //Vec recipientsRange = ctx.accounts.airdrop.recipientsRange;
        //if(amountLen >= recipientsRange[0] && amountLen <= recipientsRange[1]){
           // multiplier = 1;
        //}

        // if fee < ctx.accounts.airdrop_account.airdropFee*multiplier {
        //     return Err(ErrorCode::WrongAmount.into())
        // };

        let cpi_accounts = Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.pool_sol.to_account_info(),
            authority: ctx.accounts.payer.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

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
    pub authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

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

#[derive(Accounts)]
pub struct SendToken<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 72
    )]
    pub multi_sender: ProgramAccount<'info, TransactionAccount>,

    #[account(signer)]
    pub distribution_authority: AccountInfo<'info>,

    #[account(mut, constraint = creator_token_account.owner == *distribution_authority.key)]
    pub creator_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub receiver_authority: AccountInfo<'info>,

    #[account(mut, 
        constraint = taker_token_account.owner == *receiver_authority.key
    )]
    pub taker_token_account: Account<'info, TokenAccount>,
    
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}




#[account(zero_copy)]
#[derive(Default)]
pub struct AirdropAccount {
    pub royalty_token_account: Pubkey,
    pub airdrop_signer: Pubkey,
    pub airdrop_mint: Pubkey,
    pub authority: Pubkey,
    pub payment_mint: Pubkey,
    pub royalty_amount: u64,
    pub bump: AirdropBumps,
    pub airdrop_fee: u64,
    //pub recipientsRange: Vec[u32,8],
}
#[account]
pub struct TransactionAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Request TokenAmount must little small than total Amount of account")]
    LowTokenAmount,
    #[msg("Restricted not owner")]
    NotOwner,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct AirdropBumps {
    pub release: u8,
    pub signer: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct TransactionData {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct TransactionAccount {
//     /// The mint associated with this account
//     pub mint: Pubkey,
//     /// The owner of this account.
//     pub owner: Pubkey,
//     /// The amount of tokens this account holds.
//     pub amount: u64
//     /// If `delegate` is `Some` then `delegated_amount` represents
//     /// the amount authorized by the delegate
// //    pub delegate: COption<Pubkey>,
//     /// The account's state
// //    pub state: AccountState,
//     /// If is_some, this is a native token, and the value logs the rent-exempt reserve. An Account
//     /// is required to be rent-exempt, so the value is used by the Processor to ensure that wrapped
//     /// SOL accounts do not drop below this threshold.
// //    pub is_native: COption<u64>,
//     /// The amount delegated
// //    pub delegated_amount: u64,
//     /// Optional authority to close the account.
// //    pub close_authority: COption<Pubkey>,
// }