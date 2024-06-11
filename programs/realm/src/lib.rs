pub(crate) mod accounts_ix;
pub mod common;
pub mod constants;
pub(crate) mod ix;
pub(crate) mod macros;
pub mod state;
pub(crate) mod utils;

use crate::{
    common::Direction,
    state::{Character, Config, Realm, Resource, Skill},
};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
use mpl_core::Asset;

declare_id!("BQ2dxnsVFhfehHGS3sBy7hLrpwCJDLDsxbvzDSPY7U7x");

#[program]
pub mod realm {
    use mpl_core::instructions::CreateCollectionV1Builder;
    use solana_program::program::invoke_signed;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;

        config.bump = [ctx.bumps.config];
        config.authority = ctx.accounts.authority.key();

        Ok(())
    }

    // World building instructions
    //
    // Create the realm &/ the world
    // Create the skills that characters have

    /// Entrypoint function for [Realm] creation.
    pub fn create_realm(ctx: Context<CreateRealm>) -> Result<()> {
        let realm = &mut ctx.accounts.realm;

        realm.bump = [ctx.bumps.realm];
        Ok(())
    }
    /// Entrypoint function for [Skill] creation.
    pub fn create_skill(ctx: Context<CreateSkill>, name: [u8; 16]) -> Result<()> {
        let skill = &mut ctx.accounts.skill;

        skill.bump = [ctx.bumps.skill];
        skill.name = name;
        Ok(())
    }
    /// Entrypoint function for [Resource] creation.
    pub fn create_resource(ctx: Context<CreateResource>, resource_number: u64) -> Result<()> {
        let resource = &mut ctx.accounts.resource;

        resource.bump = [ctx.bumps.resource];
        resource.resource_number = resource_number.to_le_bytes();
        Ok(())
    }

    pub fn create_item(ctx: Context<CreateItem>) -> Result<()> {
        let ix = CreateCollectionV1Builder::new()
            .collection(ctx.accounts.collection.key())
            .update_authority(Some(ctx.accounts.authority.key()))
            .payer(ctx.accounts.payer.key())
            .system_program(ctx.accounts.system_program.key())
            .instruction();

        let account_infos = [
            ctx.accounts.collection.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        let seeds = ctx.accounts.config.get_signer_seeds();

        let signer_seeds = &[&seeds[..]];

        invoke_signed(&ix, &account_infos, &signer_seeds[..])?;
        Ok(())
    }

    // Player focused instructions
    //
    // Create the character & the inventory.

    /// Entrypoint function for [Character] creation.
    pub fn create_character(ctx: Context<CreateCharacter>) -> Result<()> {
        let character = &mut ctx.accounts.character;

        character.bump = [ctx.bumps.character];
        character.owner = ctx.accounts.owner.key();

        Ok(())
    }
    /// Entrypoint function for [Inventory] creation.
    pub fn create_inventory(ctx: Context<CreateInventory>) -> Result<()> {
        let inventory = &mut ctx.accounts.inventory;

        inventory.bump = [ctx.bumps.inventory];
        inventory.owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn move_towards(ctx: Context<MoveTowards>, direction: Direction, units: u64) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.move_towards(direction, units)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateItem<'info> {
    pub config: Box<Account<'info, Config>>,

    /// CHECK: Not checked yet but whaever.
    pub item: AccountInfo<'info>,

    /// CHECK: Checked via CPI to [mpl_core].
    pub collection: AccountInfo<'info>,

    /// CHECK: Checked via CPI to [mpl_core].
    pub asset: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [Config::SEED.as_ref()],
        bump,
        space = Config::LEN,
        payer = payer,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveTowards<'info> {
    #[account(mut)]
    pub character: Box<Account<'info, Character>>,
}

#[derive(Accounts)]
pub struct CreateCharacter<'info> {
    #[account(
        init,
        seeds = [
            Character::SEED.as_ref(),
            owner.key.as_ref()
        ],
        space = Character::ACCOUNT_LEN,
        bump,
        payer = payer
    )]
    pub character: Box<Account<'info, Character>>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateInventory<'info> {
    #[account(
        init,
        seeds = [
            Character::SEED.as_ref(),
            owner.key.as_ref()
        ],
        space = Character::ACCOUNT_LEN,
        bump,
        payer = payer
    )]
    pub inventory: Box<Account<'info, Character>>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(
        init,
        seeds = [Realm::SEED.as_ref()],
        space = Realm::ACCOUNT_LEN,
        bump,
        payer = payer
    )]
    pub realm: Box<Account<'info, Realm>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 16])]
pub struct CreateSkill<'info> {
    #[account(
        init,
        seeds = [Skill::SEED.as_ref(), name.as_ref()],
        space = Skill::ACCOUNT_LEN,
        bump,
        payer = payer
    )]
    pub skill: Box<Account<'info, Skill>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(resource_number: u64)]
pub struct CreateResource<'info> {
    #[account(
        init,
        seeds = [Resource::SEED.as_ref(), resource_number.to_le_bytes().as_ref()],
        space = Resource::ACCOUNT_LEN,
        bump,
        payer = payer
    )]
    pub resource: Box<Account<'info, Resource>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
