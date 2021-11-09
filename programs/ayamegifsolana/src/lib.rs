use anchor_lang::prelude::*;

declare_id!("7fFqkK9ScNdwZnGoVhCBjc1zMynoiND95mcQWqr8tamR");

#[program]
pub mod ayamegifsolana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;

        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct.
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *base_account.to_account_info().key,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// Attach certain variables to Initialize context
#[derive(Accounts)]
pub struct Initialize<'info> {
    // Tell Solana we want to initialize Base Account
    // init tells Solana to create new account own by our current program
    // payer = user tells our program who is paying the account
    // space = 9000 is allocation 9000 bytes of space for the account
    #[account(init, payer = user, space= 9000)]
    pub base_account: Account<'info, BaseAccount>,

    // prove that user calling this program actually owns their wallet account
    #[account(mut)]
    pub user: Signer<'info>,

    // Reference to SystemProgram; System Program basically runs Solana.
    pub system_program: Program<'info, System>,
}

// Access to mutable reference to base_account to be able top change total_gifs value in BaseAccount
#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Create a custom struct for us to work with.
// Serialize our data into binary format before storing into the "account"
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct{
    pub gif_link: String,
    pub user_address: Pubkey,
}


// Tell Solana what we want to store on this account
// Tells our program what kinda account it can make and what to hold inside
#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
