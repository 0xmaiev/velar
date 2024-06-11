use anchor_lang::prelude::*;

/// Represents a resource that a [`Character`] can harvest.
#[account]
pub struct Resource {
    /// The bump of the resource account program derived address.
    pub bump: [u8; 1], // 1
    // Padding
    pub(crate) _padding: [u8; 7], // 8

    /// The resource number.
    pub resource_number: [u8; 8], // 16
}

impl Resource {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub const ACCOUNT_LEN: usize = Self::LEN + 8;

    pub const SEED: &'static str = "resource";

    #[cfg(feature = "client")]
    pub fn derive(resource_number: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[Self::SEED.as_ref(), resource_number.to_le_bytes().as_ref()],
            &crate::id(),
        )
    }

    pub fn get_signer_seeds(&self) -> [&[u8]; 3] {
        [Self::SEED.as_ref(), &self.resource_number, &self.bump]
    }
}
