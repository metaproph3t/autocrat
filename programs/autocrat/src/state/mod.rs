use anchor_lang::prelude::*;
use anchor_lang::solana_program::{native_token::LAMPORTS_PER_SOL, pubkey};

pub use amm::*;
pub use amm_position::*;
pub use dao::*;
pub use proposal::*;

pub mod amm;
pub mod amm_position;
pub mod dao;
pub mod proposal;

pub const SLOTS_PER_10_SECS: u64 = 25;
pub const PROPOSAL_DURATION_IN_SLOTS: u64 = 5 * 24 * 60 * 6 * SLOTS_PER_10_SECS;

// by default, the pass price needs to be 5% higher than the fail price
pub const DEFAULT_PASS_THRESHOLD_BPS: u64 = 500;

// start at 10 SOL ($1000 at current prices), decay by ~5 SOL per day
pub const DEFAULT_BASE_BURN_LAMPORTS: u64 = 10 * LAMPORTS_PER_SOL;
pub const DEFAULT_BURN_DECAY_PER_SLOT_LAMPORTS: u64 = 23_150;

pub const AMM_INITIAL_QUOTE_LIQUIDITY_ATOMS: u64 = 500 * 1_000_000; // $500 * 10^6

pub const AMM_SWAP_FEE_BPS: u64 = 300; // 3%
pub const AMM_SWAP_FEE_BPS_MIN: u64 = 100; // 1%
pub const AMM_SWAP_FEE_BPS_MAX: u64 = 1000; // 10%

pub const BPS_SCALE: u64 = 100 * 100;
