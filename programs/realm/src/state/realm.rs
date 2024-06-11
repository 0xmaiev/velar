use anchor_lang::prelude::*;

#[account]
pub struct Realm {
    // the realm's bump
    pub bump: [u8; 1],

    // the realm's admin
    pub admin: Pubkey,
}

impl Realm {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub const ACCOUNT_LEN: usize = Self::LEN + 8;

    pub const SEED: &'static str = "realm";

    #[cfg(feature = "client")]
    pub fn derive() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::SEED.as_ref()], &crate::id())
    }

    pub fn get_signer_seeds(&self) -> [&[u8]; 2] {
        [Self::SEED.as_ref(), &self.bump]
    }
}
