use crate::common::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug, Default)]
pub struct Character {
    pub bump: [u8; 1],
    pub(crate) padding: [u8; 7],

    pub owner: Pubkey,

    pub position: Position,
}

impl Character {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub const ACCOUNT_LEN: usize = Self::LEN + 8;

    pub const SEED: &'static str = "character";

    #[cfg(feature = "client")]
    pub fn derive(owner: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::SEED.as_ref(), owner.as_ref()], &crate::id())
    }

    pub fn get_signer_seeds(&self) -> [&[u8]; 2] {
        [Self::SEED.as_ref(), &self.bump]
    }

    pub fn move_towards(&mut self, direction: Direction, units: u64) -> Result<()> {
        // todo: rework this to add checks on tile bounds
        // todo: rework this to make the `moving` action non-instant
        match direction {
            Direction::North => {
                self.position.y += units;
            }
            Direction::East => {
                self.position.x += units;
            }
            Direction::South => {
                self.position.y -= units;
            }
            Direction::West => {
                self.position.x -= units;
            }
        }
        Ok(())
    }
}
