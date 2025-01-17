# nft-staking

This example shows how to stake a NFT using the Token Metadata Program.
A user will be able to stake a NFT of a certain colleciton, get points, and mint rewards.

---

## Let's walk through the architecture:

For this program, we will have 3 state accounts:
- A User account
- A Staking Config account
- A Stake account

A User account consists of:

```rust
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u8,
    pub bump: u8,
}
```

In this User state account, we store:
- points: The current points that the user has
- amount_staked: The number of NFTs that the user has currently staked.
- bump: we will store the bump of the account because User account will be a PDA.

---

A Stake Config account consists of:

```rust
#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_period: u32,
    pub rewards_bump: u8,
    pub bump: u8,
}
```
In this Stake Config state account, we store:
- points_per_stake: The points that the user will receive per stake.
- freeze_period: The time that the NFT need to be staked before being unstaked.
- rewards_bump: We will be initializing a rewards mint based on an PDA, so we will store that PDA bump.
- bump: we store the bump of the account because Stake Config is a PDA.

---

```rust
#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8,
}
```
In this Stake Account, we store:
- owner: The owner of this account.
- mint: To address of the NFT staked.
- staked_at: The time the NFT was staked.
- bump: we store the bump of the account because Stake Account is a PDA.

---

#### The user will be able to create new User accounts. For that, we create the following context:

```
#[derive(Accounts)]
pub struct Initialize<'info>> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = 8 + UserAccount::INITSPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
```
In this context:
- user: Will be the person starting the user account, the user will be a signer of the transaction, and we mark this user account as mutable as we will be deducting lamports from this account.

- user_account: Will be the state account that we will initialize and the user will be paying for the initialization of the account. We derive the User PDA from the byte representation of the word "user" and the reference of the user public key. Anchor will calculate the bump. The first bump that throws that address out of the ed25519 eliptic curve. The bump will be stored in a struct.

---

#### The admin of the platform will be able to create Config accounts:

```rust
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"config".as_ref()],
        space = 8 + StakeConfig::INIT_SPACE,
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub rewards_mint: Account<'info, Mint>,
    pub token_program: Program<'info, System>,
    pub system_program: Program<'info, System>,
}
```
In this context, we are passing all the accounts needed to contribute to fundraising campaign:
- Admin: The address of the platform admin. He will be a signer of the transaction, and we mark admin account as mutable as we will be deducting lamports from this account.

- config: Will be the state account that we will initialize and the admin will be paying for the initialization of the account. We derive the Config PDA from the byte representation of the word "config".

- rewards_mint: We initialize a rewards mint account that will be used to mint rewards to the user. We initialize it with six decimals, and the authority of that mint account will be the config account so that our program can mint tokens.

- token_program: We are initializing a Mint account.

- system_program: Program responsible for the initialization of any new account.

#### The user will be able to stake NFTs with this context:

```rust
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint_key().as_ref(),
        ],
        seeds:program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap.key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Accont<'info, MasterEditionAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,

}
```

In this context, we are passing all the accounts needed for a user to stake NFT:
- user: The address of the user staking the NFT. It is mutable since the user will be paying for initialization fees.
- mint: the NFT that the user is staking.
- collection_mint: The collection NFT to which the NFT being staked belongs to.
- mint_ata: The user ATA(Associated Token Account) where the NFT will be frozen.
- metadata: The Metadata accunt of the NFT. In here, we chec, that this metadata account belongs to the NFT being staked. We also check that the NFT belongs to the correct collection and that is verified as part of that collection.
- edition: The Master Edition account of the NFT.
- config: The Stake Config acount being used to stake this NFT.
- stake_account: Will be the state account that we will initialize and the user will be paying for the initialization of the account. We derive the Stake Account PDA.

---
#### Users will be able to unstake their contributions with this context:
```rust
#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        close = user,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}
```
In this context, we are passing all the accounts needed for a contributor to unstake his NFT:

- user: The address of the user that wants to unstake the NFT.
- mint: The NFT being unstaked.
- mint_ata: The user ATA(Associated Token Account) where the NFT is frozen.
- edition: The NFT Master Edition account.
- config: An initialized Stake Config account that will ne used to perform checks and award points.
- stake_account: The NFT Stake Account. We will be closing this account and sending the rent back to the user.
- user_account: The User account where will be updating the total amount staked and the total points.
- token_program: We will performing CPIs (Cross Program Invocations) to the token program to revoke authority over the NFT.
- metadata_program: We will be performing CPIs to the token metadata program to thaw / unfreeze the NFT.
- system_program: We will be closing accounts.

---

#### Users will be able to clain their rewards:

```rust
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
```
In this context, we are passing all the accounts needed for a contributor to unstake his NFT:
- user: The user claiming the rewards. We mark it as mutabçe has it might need to pay for initialization fees.
- user_account: The User account, derived from the user public key.
- rewards_mint: The rewards mint that we will be minting to the user.
- config: The Config account that has authority over the rewards mint.
- rewards_ata: The user rewards ATA (associated token account). If it doesn't exist we will initialize it and the user will pay for the initialization fees.
- system_program: The program responsible for initializating any account.
- token_program: We will be transfering SPL Tokens.
- associated_token_program: We might need to initialize an ATA (associated token account).

## How to test on localnet
`solana program dump --url mainnet-beta metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

`solana-test-validator --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --reset`

`anchor deploy`

`anchor test --skip-local-validator --skip-deploy`

### Test Result
```
console.log
created Collection NFT:  2N2gqgGNHCgi3PxTH5yQwLeGbRPP5f3RVxN9h9Pe1qo4

console.log    
Created NFT: 6CKd97vGrEiDiotaR8sR1N5nPgewjiCzxVRQ2cbHiMFC

console.log    
Collection NFT Verified!

console.log
Config Account Initialized!

console.log
Your transaction signature 3q3R4mW9RfEWo4v6MjUeWw8R8UeQLs66iPkRUkxtimaimy1GoQYs7VHVJWjqVkmpwP6hnL9N5MQAQxudHRAJKHFQ

console.log    
User Account Initialized!

console.log
Your transaction signature 4kYFw8gowEWtzNHuxWveVVhTDVeEmkWYyCfLPYwMER3h7MBrowHYbiPW93R9r6t19UTX7hHKqibDEwJbSKXjweJr

console.log    
NFT Staked!

console.log
Your transaction signature 5NVwRmykfUjoiQ8Yhk3YqsGYJDH52LZbdBDvADhkJ4C5V7xFm5u5J9md5UjkmtSbrrHxGdgQEZr3JZty7gRfAUjx

console.log    
NFT unstaked!

console.log
Your transaction signature 2eNZJ3aCH71j3RaTGLAMwQwm4S8Zem6xC6uCfTzLQQ9FrrQXswSY7Jmj4KABc2VUqotmAL3CVpBWamsyathDxhu7

console.log
user points:  0

console.log
Rewards claimed

console.log
Your transaction signature 3idpBTHqH4AdrShYizPMgfnbriY76m36Vm32o2r2b6wYjTsfnZobHEmCJAVNJvjcoU6zKkSkSwp3hWG5rTGTMWps

console.log
User points:  0

```
