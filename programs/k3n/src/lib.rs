use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::transfer;
use std::ops::DerefMut;

declare_id!("7EsEmSeRQ98ni2qed55nfMFYyD1omasLPsRn7jeWL7jF");

#[program]
pub mod k3n {
    use super::*;

    pub fn create_service(
        ctx: Context<CreateService>,
        title: String,
        description: String,
        price: u64,
        payment_type: PaymentType,
    ) -> Result<()> {
        let new_service = ctx.accounts.service_information.deref_mut();
        *new_service = ServiceInformation {
            title,
            description,
            price,
            payment_type,
            owner: *ctx.accounts.user.key,
        };
        Ok(())
    }
    // Function to buy a service
    pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
        let service = &ctx.accounts.service_information;
        let owner = &ctx.accounts.service_information.owner;
        let buyer = &ctx.accounts.buyer;

        // Calculate the lamports to transfer based on the service price
        let lamports = service.price;

        // Perform the transfer from the buyer to the service owner
        **owner.try_borrow_mut_lamports()? += lamports;
        **buyer.try_borrow_mut_lamports()? -= lamports;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateService<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub service_information: Account<'info, ServiceInformation>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ServiceInformation {
    pub title: String,
    pub description: String,
    pub price: u64,
    pub payment_type: PaymentType,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PaymentType {
    OnetimePayment,
    Milestone,
    //Won { winner: Pubkey },
}
#[derive(Accounts)]
pub struct BuyService<'info> {
    #[account(mut)]
    pub service_information: Account<'info, ServiceInformation>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Error handling
// #[error]
// pub enum Error {
//     #[msg("Insufficient funds")]
//     InsufficientFunds,
// }
pub fn transfer_sol(ctx: Context<TransferSol>, lamports: u64) -> Result<()> {

  let ix = anchor_lang::solana_program::system_instruction::transfer(
    &ctx.accounts.from.key(),
    &ctx.accounts.to.key(),
    amount,
  );
  anchor_lang::solana_program::program::invoke(
    &ix,
    &[
      ctx.accounts.from.to_account_info(),
      ctx.accounts.to.to_account_info(),
    ],
  );



  let ix = Transfer {
    from_pubkey: ctx.accounts.from.key(),
    to_pubkey: ctx.accounts.to.key(),
    lamports,
  };

  anchor_lang::solana_program::program::invoke(
    &ix,
    &[
      ctx.accounts.from.to_account_info(),
      ctx.accounts.to.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
    ],
  )?;
  Ok(())
}
