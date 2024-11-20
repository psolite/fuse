use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

// pub const TOKEN_METADATA_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0x41, 0x61, 0x54, 0x4c, 0x48, 0x69, 0x63, 0x4b, 0x75, 0x55, 0x57, 0x35, 0x6f, 0x48, 0x58, 0x6e, 0x44, 0x77, 0x69, 0x54, 0x70, 0x34, 0x41, 0x67, 0x73, 0x43, 0x37, 0x70, 0x56, 0x73, 0x53, 0x59]);

declare_id!("rtacvJhGMzaCHjAgscGQgQSZBveyP7jgzRkjSztoEd6");

#[program]
pub mod fuse {

    use super::*;

    // Create a new mint
    pub fn create_mint(ctx: Context<CreateMint>, meta_data: TokenMetadata) -> Result<()> {
        let token_details = &mut ctx.accounts.token_details;
        let seeds = &["fuse".as_bytes(), &[ctx.bumps.mint]];
        let signer = &[&seeds[..]];

        let token_data: DataV2 = DataV2 {
            name: meta_data.name,
            symbol: meta_data.symbol,
            uri: meta_data.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metadata_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.mint.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer,
        );

        create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

        token_details.mint = *ctx.accounts.mint.to_account_info().key;
        token_details.owner = *ctx.accounts.payer.to_account_info().key;

        msg!("Mint created successfully by Psolite Fuse.");

        Ok(())
    }

    // Mint tokens to a user
    pub fn mint_tokens(ctx: Context<MintToAccount>, amount: u64) -> Result<()> {
        let seeds = &["fuse".as_bytes(), &[ctx.bumps.mint]];
        let signer = &[&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        msg!("Tokens minted successfully by Psolite Fuse.");

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(meta_data: TokenMetadata)]
pub struct CreateMint<'info> {
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    // #[account(
    //     init,
    //     space = 829,
    //     seeds = [b"metadata", token_metadata_program.key().as_ref(), mint.key().as_ref()],
    //      bump,
    //      payer = payer
    // )]
    // pub metadata: AccountInfo<'info>,

    #[account(
        init,
        seeds = [b"fuse"],
        bump,
        payer = payer,
        mint::decimals = meta_data.decimals,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer, space = 8 + 32 + 32,
        seeds = [b"token", payer.key().as_ref()],
        bump
    )]
    pub token_details: Account<'info, TokenDetails>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
 
    // pub token_metadata_program: AccountInfo<'info>,
    pub token_metadata_program: Program<'info, Metaplex>,
}

#[derive(Accounts)]
pub struct MintToAccount<'info> {
    #[account(
        mut,
        seeds = [b"fuse"],
        bump,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct TokenDetails {
    pub mint: Pubkey,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}
