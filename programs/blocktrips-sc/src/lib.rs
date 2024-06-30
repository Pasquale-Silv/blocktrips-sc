use anchor_lang::prelude::*;

declare_id!("Eu6oXy1qR4xBfxNphKzeBMA41tT3EycJRs7wVNCNSnDN");

#[program]
pub mod blocktrips_sc {
    use super::*;

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
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = 8 + 4 + 32 + 32 + 1 + 8 + 4 + 8 + 4 + 8,
        payer = payer
    )]
    pub trip: Account<'info, Trip>,
    pub system_program: Program<'info, System>,
}

#[account]
// #[derive(InitSpace)]
pub struct Trip {
    review: String,
    traveler: Pubkey,
    accommodation_business: Pubkey,
    is_for_sale: bool,
    price: f64,
    date_of_departure: String,        // YYYYMMDD
    end_date: String,                 // YYYYMMDD
}
