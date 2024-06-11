use anchor_lang::prelude::*;

/// Represents a skill that a [`Character`] possesses.
#[account]
pub struct Skill {
    /// The bump of the skill account program derived address.
    pub bump: [u8; 1], // 1
    // Padding
    pub(crate) _padding: [u8; 7], // 8

    /// The name of the skill.
    ///
    /// Encoded as zero-padded UTF-8 string.
    pub name: [u8; 16], // 24
    /// The maximum level of the skill.
    pub max_level: u16, // 26

    // Padding.
    pub(crate) _padding2: [u8; 6], // 32
    /// The maximum experience attainable in the skill.
    pub max_experience: u128, // 48
}

impl Skill {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub const ACCOUNT_LEN: usize = Self::LEN + 8;

    pub const SEED: &'static str = "skill";

    #[cfg(feature = "client")]
    pub fn derive(name: &[u8; 16]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[Self::SEED.as_ref(), name], &crate::id())
    }

    pub fn get_signer_seeds(&self) -> [&[u8]; 3] {
        [Self::SEED.as_ref(), &self.name, &self.bump]
    }
}
