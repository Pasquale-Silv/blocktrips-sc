use anchor_lang::prelude::*;

declare_id!("Eu6oXy1qR4xBfxNphKzeBMA41tT3EycJRs7wVNCNSnDN");

#[program]
pub mod blocktrips_sc {
    use super::*;

    // This function is used by the Accommodation Business to create a Trip for selling it at a specified price
    pub fn initialize(
        ctx: Context<Initialize>,
        accommodation_business: Pubkey,
        date_of_departure: String,
        end_date: String,
        price: f64
    ) -> Result<()> {
        msg!("Creating the trip...");
        let trip = &mut ctx.accounts.trip;
        trip.accommodation_business = accommodation_business;
        trip.date_of_departure = date_of_departure;
        trip.end_date = end_date;
        trip.price = price;
        trip.is_for_sale = true;
        Ok(())
    }

    // This function is used by the Traveler to buy the Trip
    pub fn buy(
        ctx: Context<BuyTrip>,
        traveler: Pubkey,
        price: u64
    ) -> Result<()> {
        msg!("Buying the trip...");

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            price,
        );

        let _ = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        );

        msg!("SOL sent...");

        let trip = &mut ctx.accounts.trip;
        trip.traveler = traveler;
        trip.is_for_sale = false;
        Ok(())
    }

    // This function is used by the Traveler to put the Trip up for sale
    pub fn put_up_for_sale(
        ctx: Context<PutTripUpForSale>,
        price: f64
    ) -> Result<()> {
        msg!("Putting the Trip up for sale...");
        let trip = &mut ctx.accounts.trip;
        trip.is_for_sale = true;
        trip.price = price;
        Ok(())
    }

    pub fn set_price(
        ctx: Context<UpdatePrice>,
        price: f64
    ) -> Result<()> {
        msg!("Updating the price of the trip...");
        let trip = &mut ctx.accounts.trip;
        trip.price = price;
        Ok(())
    }

    // Function to Close/Eliminate the Trip
    pub fn close(_ctx: Context<CloseTrip>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = 8 + 4 + 32 + 32 + 1 + 8 + 4 + 10 + 4 + 10,
        payer = payer
    )]
    pub trip: Account<'info, Trip>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub trip: Account<'info, Trip>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTrip<'info> {
    #[account(mut)]
    pub trip: Account<'info, Trip>,
    #[account(mut)]
    pub from: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PutTripUpForSale<'info> {
    #[account(mut)]
    pub trip: Account<'info, Trip>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseTrip<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub trip: Account<'info, Trip>,
}

#[account]
// #[derive(InitSpace)]
pub struct Trip {
    review: String,
    traveler: Pubkey,
    accommodation_business: Pubkey,
    is_for_sale: bool,
    price: f64,
    date_of_departure: String,        // YYYY-MM-DD, I.e.: 2024-06-06
    end_date: String,                 // YYYY-MM-DD
}
