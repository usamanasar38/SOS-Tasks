//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove comment functionality for the Twitter program
/// 
/// Requirements:
/// - Close the comment account and return rent to comment author
/// 
/// NOTE: No implementation logic is needed in the function body - this 
/// functionality is achieved entirely through account constraints!
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::states::*;

pub fn remove_comment(_ctx: Context<RemoveCommentContext>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        mut,
        close = comment_author, // Close the comment account and return rent to the author
        constraint = comment.comment_author == comment_author.key(), // Ensure the comment belongs to the author
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash(comment.content.as_bytes()).to_bytes().as_ref()}, // Use content hash for unique identification
            comment.parent_tweet.key().as_ref(),
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
}
