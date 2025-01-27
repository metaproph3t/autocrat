use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::error::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateProposalPartTwo<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"proposal",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        mut,
        seeds = [
            b"pass_market_amm",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub pass_market_amm: Box<Account<'info, Amm>>,
    #[account(
        mut,
        seeds = [
            b"fail_market_amm",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub fail_market_amm: Box<Account<'info, Amm>>,
    #[account(
        constraint = meta_mint.key() == proposal.meta_mint.key()
    )]
    pub meta_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = usdc_mint.key() == proposal.usdc_mint.key()
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            b"conditional_on_pass_meta",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub conditional_on_pass_meta_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            b"conditional_on_pass_usdc",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub conditional_on_pass_usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            b"conditional_on_fail_meta",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub conditional_on_fail_meta_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            b"conditional_on_fail_usdc",
            proposal.number.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub conditional_on_fail_usdc_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = meta_mint,
        associated_token::authority = proposer,
    )]
    pub meta_proposer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = proposer,
    )]
    pub usdc_proposer_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = meta_mint,
        associated_token::authority = proposal,
    )]
    pub meta_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = proposal,
    )]
    pub usdc_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = conditional_on_pass_meta_mint,
        associated_token::authority = proposal,
    )]
    pub conditional_on_pass_meta_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = conditional_on_pass_usdc_mint,
        associated_token::authority = proposal,
    )]
    pub conditional_on_pass_usdc_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = conditional_on_fail_meta_mint,
        associated_token::authority = proposal,
    )]
    pub conditional_on_fail_meta_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = conditional_on_fail_usdc_mint,
        associated_token::authority = proposal,
    )]
    pub conditional_on_fail_usdc_vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateProposalPartTwo>,
    initial_pass_market_price_units: f32, // human-readable price (i.e. units)
    initial_fail_market_price_units: f32, // human-readable price (i.e. units)
    quote_liquidity_atoms_per_amm: u64,
) -> Result<()> {
    let CreateProposalPartTwo {
        proposer,
        proposal,
        pass_market_amm,
        fail_market_amm,
        meta_mint,
        usdc_mint,
        conditional_on_pass_meta_mint,
        conditional_on_pass_usdc_mint,
        conditional_on_fail_meta_mint,
        conditional_on_fail_usdc_mint,
        meta_proposer_ata,
        usdc_proposer_ata,
        meta_vault_ata,
        usdc_vault_ata,
        conditional_on_pass_meta_vault_ata,
        conditional_on_pass_usdc_vault_ata,
        conditional_on_fail_meta_vault_ata,
        conditional_on_fail_usdc_vault_ata,
        associated_token_program: _,
        token_program: _,
        system_program: _,
    } = ctx.accounts;

    assert!(proposal.part_one_complete);
    assert!(!proposal.part_two_complete);
    proposal.part_two_complete = true;

    let clock = Clock::get()?;
    proposal.slot_enqueued = clock.slot;

    pass_market_amm.ltwap_slot_updated = clock.slot;
    fail_market_amm.ltwap_slot_updated = clock.slot;

    // ==== deposit initial liquidity ====
    // TODO

    Ok(())
}
