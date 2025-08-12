//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    require!(comment_content.len() <= COMMENT_LENGTH, TwitterError::CommentTooLong);

    let comment_author = &ctx.accounts.comment_author;
    let tweet = &ctx.accounts.tweet;

    let comment = &mut ctx.accounts.comment;
    comment.comment_author = comment_author.key();
    comment.content = comment_content;
    comment.parent_tweet = tweet.key();
    comment.bump = ctx.bumps.comment;
    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init, 
        payer = comment_author, 
        // space = discriminant + account size
        space = 8 + Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash(comment_content.as_bytes()).to_bytes().as_ref()},
            tweet.key().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
