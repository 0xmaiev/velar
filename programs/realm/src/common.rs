use anchor_lang::prelude::*;

#[derive(Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Default, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}
