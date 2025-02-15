# Storage in Solana

## Cost of Storage

* When accounts are initialized, the amount of rent needed is computed in the background

* If you want a quick estimate, running `solana rent <number of bytes>` in the command line will give you a quick answer.


* The Anchor Rent Module gives us some constants related to rent:
  * ACCOUNT_STORAGE_OVERHEAD: this constant has a value of 128 (bytes) and as the name suggests, an empty account has 128 bytes of overhead.
  * DEFAULT_EXEMPTION_THRESHOLD: this constant has a value of 2.0 (float 64) and refers to the fact that paying two years of rent in advance makes the account exempt from paying further rent.
  * DEFAULT_LAMPORTS_PER_BYTE_YEAR: this constant has a value of 3,480 meaning each byte requires 3,480 lamports per year. Since we are required to pay two years worth, each byte will cost us 6,960 lamports.

* Solana has a permanent inflation model that will eventually converge to 1.5% per year, so this should help reflect the fact that storage gets cheaper over time per Moore’s Law, which states that transistor density for the same cost doubles every 18 months.

* If a wallet ends up getting deleted due to having the balance below the rent exempt threshold, it can be “resurrected” by sending more SOL to it, but if data is stored in the account, that data will be lost.

<br>

##  Maximum Storage Size


* Size limitations
  When we initialize an account, we cannot initialize more than 10,240 bytes in size.

<br>

##  Account Resizing

* Changing the size of an account
If you need to increase the size of the account, we can use the `realloc` macro.

* When increasing the size of the account, be sure to set `realloc::zero = false` (in the code above) if you do not want the account data erased. 

* The maximum account size increase per realloc is 10240. The maximum size an account can be in Solana is 10 MB.

* Anticipating the cost of deploying a program
  * The bulk of the cost of deploying a Solana program comes from paying rent for storing the bytecode. The bytecode is stored in a separate account from the address returned from anchor deploy.

* A simple hello world program currently costs over 2.47 SOL to deploy. The cost can be significantly reduced by writing raw Rust code instead of using the Anchor framework, but we don’t recommend doing that until you fully understand all the security risks Anchor eliminates by default.

<br>

#### example
```rust
use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent as rent_module;
use std::mem::size_of;

declare_id!("YFtoALYAjN4cLjyZRjAVaP5JRoy1FvKTevJER7nBSuE");

#[program]
pub mod day_20_rent {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let cost_of_empty_acc = rent_module::ACCOUNT_STORAGE_OVERHEAD as f64 * 
                                rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
                                rent_module::DEFAULT_EXEMPTION_THRESHOLD;

        msg!("cost to create an empty account: {}", cost_of_empty_acc);
        // 890,880 lamports

        let cost_for_32_bytes = cost_of_empty_acc + 
                                32 as f64 * 
                                rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
                                rent_module::DEFAULT_EXEMPTION_THRESHOLD;

        msg!("cost to create a 32 byte account: {}", cost_for_32_bytes);
        // 1,113,600 lamports
        Ok(())
    }

    pub fn increase_account_size(ctx: Context<IncreaseAccountSize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncreaseAccountSize<'info> {

    #[account(mut,
              // ***** 1,000 BYTE INCREMENT IS OVER HERE *****
              realloc = size_of::<MyStorage>() + 8 + 1000,
              realloc::payer = signer,
              realloc::zero = false,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
```