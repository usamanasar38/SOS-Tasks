//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;


    // Increment the appropriate counter based on reaction type
    match tweet_reaction.reaction {
        ReactionType::Like => {
            tweet.likes -= 1;
        },
        ReactionType::Dislike => {
            tweet.dislikes -= 1;
        },
    }
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        payer = reaction_author, // Close the comment account and return rent to the author
        constraint = tweet_reaction.reaction_author == reaction_author.key(), // Ensure the comment belongs to the author
        seeds = [
            b"tweet_reaction",
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
