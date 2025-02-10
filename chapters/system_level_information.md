# System Level Information in Solana

## Solana Sysvars

* sysvars are read-only system accounts that give Solana programs access to the blockchain state as well as network information. 

* In Anchor programs, you can access sysvars in two ways: either by using the anchor’s get method wrapper, or by treating it as an account in your #[Derive(Accounts)], using its public address.

* Fees: Contains the fee calculator for the current slot. The fee calculator provides information on how many lamports are paid per signature in a Solana transaction.
* EpochRewards: The EpochRewards sysvar holds a record of epoch rewards distribution in Solana, including block rewards and staking rewards.
* RecentBlockhashes: Contains the active recent block hashes.
* SlotHashes: Contains history of recent slot hashes.
* SlotHistory: Holds an array of slots available during the most recent epoch in Solana, and it is updated every time a new slot is processed.
* StakeHistory: maintains a record of stake activations and deactivations for the entire network on a per-epoch basis, which is updated at the beginning of each epoch.
* Instructions: To get access to the serialized instructions that are part of the current transaction.
* LastRestartSlot: Contains the slot number of the last restart (the last time Solana restarted ) or zero if none ever happened. If the Solana blockchain were to crash and restart, an application can use this information to determine if it should wait until things stabilize.

<br>

---

### Block Timestamp in Solana

* By utilizing the unix_timestamp field within the Clock sysvar, we can access the block timestamp Solana.

```rust
#[program]
pub mod sysvar {
    use super::*;
    use chrono::*;

    pub fn get_day_of_the_week(_ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp; // get block timestamp

        let date_time = DateTime::from_timestamp(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!(
            "Block timestamp: {}", time_stamp,
        );

        msg!("Week day is: {}", day_of_the_week);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
}
```

<br>

----

### Block Number in Solana

* Solana has a notion of a “slot number” which is very related to the “block number” but is not the same thing. The distinction between these will be covered in the following tutorial, so we defer a full discussion of how to get the “block number” until then.

* Solana uses a leader-based consensus mechanism which is a combination of both Proof of History (PoH) and Proof of Stake (PoS), removing the concept of mining. Instead, a block or slot leader is appointed to validate transactions and propose blocks during certain intervals, under a system known as the leader schedule. This schedule determines who will be the block producer at a certain time.

<br>

----

### Gas fee in Solana
* Solana has a per-block compute unit limit of 48 million. Each transaction is by default limited to 200,000 compute units, though it can be raised to 1.4 million compute units (we will discuss this in a later tutorial, though you can see an example here).

* Solana, the base price of a transaction is static, so there is no need for a variable like this.

<br>

---

### changing program id depending on the clsuter.
 
* https://solana.stackexchange.com/questions/848/how-to-have-a-different-program-id-depending-on-the-cluster


<br>

---

#### Clock

* Clock: Used for time-related operations like getting the current time or slot number.
EpochSchedule: Contains information about epoch scheduling, including the epoch for a particular slot.

``` rust
    let clock = Clock::get()?;
```

<br>

---

#### EpochSchedule sysvar

* An epoch in Solana is a period of time that is approximately two days long. SOL can only be staked or unstaked at the start of an epoch.

```rust
    let epoch_schedule = EpochSchedule::get()?;
```

<br>

---

#### Rent sysvar

* Rent: Contains the rental rate and information like the minimum balance requirements to keep an account rent exempt.

```rust
let rent_var = Rent::get()?;
```

<br>

---

### Solana Logs and Events

* There is no such thing as “indexed” or “non-indexed” information in Solana like there is in Ethereum

* We cannot directly query for past events over a range of block numbers. We can only listen for events as they occur.

```rust
#[program]
pub mod emit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 3, message: "hello world".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub value: u64,
    pub message: String,
}
```

* Solana logs are not for historical querying

* In Solana, logs are run by calling the system call `sol_log_data`. As an argument, it is simply a sequence of bytes

```rust
/// Print some slices as base64.
pub fn sol_log_data(data: &[&[u8]]) { ... }
```

* Anchor turns the struct into a byte sequence to pass to this function. The Solana system call only takes a byte sequence, not a struct.

* Logs are better suited for passing information to the frontend application.

* Solana transactions can be queried by address.

* Getting the transaction history in Solana

  * Solana has an RPC function `getSignaturesForAddress` which lists all the transactions an address has done. 

  * The actual content of the transaction is retrieved using the `getParsedTransactio` RPC method.

```typescript
let web3 = require('@solana/web3.js');

const solanaConnection = new web3.Connection(web3.clusterApiUrl("mainnet-beta"));

const getTransactions = async(address,limit) => {
  const pubKey = new web3.PublicKey(address);
  let transactionList = await solanaConnection.getSignaturesForAddress(pubKey, {limit: limit});
  let signatureList = transactionList.map(transaction => transaction.signature);

  console.log(signatureList);

  for await (const sig of signatureList) {
    console.log(await solanaConnection.getParsedTransaction(sig, {maxSupportedTransactionVersion: 0}));
  }
}

let myAddress = "enter and address here";

getTransactions(myAddress, 3);
```

### #[access_control] attribute
* The #[access_control] attribute executes the given access control method before running the main instruction.

* the only owner pattern, that is, restrict a function’s access in our Solana program to a PubKey (owner’s address).

```rust
const OWNER: &str = "OWNER_PUBLIC_KEY";

#[program]
pub mod only_owner {
    use super::*;

    #[access_control(check(&ctx))]
    pub fn initialize(ctx: Context<OnlyOwner>) -> Result<()> {
        // Function logic...

        msg!("Holla, I'm the owner.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
```

<br>

---

### Compute Units and Transaction Fees

* Solana chain executes compiled bytecode and charge a fee for each instruction executed.

* Solana runs a modified version of berkeley packet filter called Solana packet filter.

* In Solana, each opcode costs one compute unit.

* When performing heavy computational operations that cannot be done below the limit, the traditional strategy is to “save your work” and do it in multiple transactions. The “save your work” part needs to be put into permanent storage.

* Compute Unit Optimization
  * Solana has a compute unit limit per transaction of 200,000 CU (can be increased up to 1.4m CU at some extra cost) which if it (the chosen limit) is exceeded, the program terminates, all changed states revert back and fees are not returned back to the caller. 
  * The number of signers for the Solana transaction affects the compute unit cost. 
  * A smaller transaction is more likely to be included in a block if there is significant network activity competing for blockspace.
  * It will make your program more composable with other programs.
  * Smaller integers save compute units
  * Generating a program derived account (PDA) on-chain using `find_program_address` may use more compute units because this method iterates over calls to `create_program_address` until it finds a PDA that's not on the ed25519 curve. To reduce the compute cost, use `find_program_address()` off-chain and pass the resulting bump seed to the program when possible

<br>

--- 

### eBPF and SBF

* eBPF simply means extended BPF.
* In a nutshell, eBPF allows execution of arbitrary eBPF bytecode within the kernel (in a sandbox environment)
* This allows us to build programs for various use cases
  * network: To analyze routes and more
  * security: filtering traffic based on certain rules and reporting any bad/blocked traffic
  * tracing and profiling: collecting detailed execution flow from the userspace program to the kernel instructions
  * observability: report and analyze kernel activities
  * The program is only executed when we need it (i.e. when an event is emitted in the kernel).
* Solana Bytecode Format (SBF)
  * Solana Bytecode Format is a variant of eBPF with certain changes and the one that stands out the most is the removal of the bytecode verifier.
  * Having a compute meter that limits computational resources spent with a cap moves safety checks to the runtime and allows arbitrary memory access, indirect jumps, loops, and other interesting behaviours.
