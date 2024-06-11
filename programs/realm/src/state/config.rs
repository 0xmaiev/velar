use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub bump: [u8; 1],
    pub(crate) padding: [u8; 7],

    /// The authority.
    pub authority: Pubkey,

    pub(crate) extra: [u8; 212],
}

impl Config {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub const SEED: &'static str = "config";

    #[cfg(feature = "client")]
    pub fn derive() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::SEED.as_ref()], &crate::id())
    }

    pub fn get_signer_seeds(&self) -> [&[u8]; 2] {
        [Self::SEED.as_ref(), &self.bump]
    }
}
