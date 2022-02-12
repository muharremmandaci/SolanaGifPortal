use anchor_lang::prelude::*;

// Anchor has generated this one for us. We'll be changing it later.
declare_id!("8nGuYmJjCbcsdueLd7AcuyMwMjvAoWVEQxLUymf6VfxN");

#[program]
pub mod myepicproject {
  use super::*;
  // takes something called a Context and outputs a ProgramResult
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }
	// Another function woo!
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      //gif_vote: 0,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
/* 
  	// Another function woo!
    pub fn add_vote(ctx: Context<AddVote>, gif_link: String) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;
      let gif_link = gif_link.to_string();
  
      
    // Add it to the gif_list vector.
      //base_account.gif_list.push(item);
      //base_account.total_gifs += 1;

      if let Some(index) = base_account.gif_list.iter().position(|&i| i.gif_link == gif_link) {
        base_account.gif_list.gif_vote +=1;
      }

      Ok(())
    }*/
}

/*
1 - init will tell Solana to create a new account owned by our current program.
2 - payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
3 - We then say space = 9000 which will allocate 9000 bytes of space for our account. You can change this # if you wanted, 
    but, 9000 bytes is enough for the program we'll be building here!
*/

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // All we're doing here is telling Solana how we want to initialize BaseAccount
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    // We then have pub user: Signer<'info> which is data passed into the program 
    // that proves to the program that the user calling this program actually owns their wallet account.
    pub user: Signer<'info>,
    // SystemProgram -> one of the main things it does is create accounts on Solana
    pub system_program: Program <'info, System>,
}



// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}
/* 
#[derive(Accounts)]
pub struct AddVote<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}*/

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    //pub gif_vote:u64,
}

// Tell Solana what we want to store on this account.
// So, here, BaseAccount holds one thing and it's an integer named total_gifs
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}