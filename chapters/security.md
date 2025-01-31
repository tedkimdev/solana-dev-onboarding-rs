# Writing Secure Programs on Solana (WIP)


## Common Security Exploits

<br>

#### Missing Ownership/Address Check

* Verify that an account is owned by the expected program by using Anchor's `Account<'info, T>` type that checks the owner (instead of `AccountInfo<'info>`).

* Issue: When you use `AccountInfo<'info>`, the contract does not inherently check if the account is owned by the program you expect. This can lead to vulnerabilities where malicious users can pass in accounts owned by other programs, potentially leading to unauthorized operations.
* Solution: Use `Account<'info, T>`, which enforces that the account is owned by the expected program (T), ensuring that only accounts that the program should interact with are used.
<br>

#### Missing Signer Check

* This vulnerability occurs when an account is not signed so anyone who knows the user pubkey can use it in a transaction.
* A solution is to replace `AccountInfo<'info>` with `Signer<'info>`.
* Issue: `AccountInfo<'info>` does not verify if the account has been signed by the corresponding private key in the transaction. This can allow attackers to submit transactions using other users' accounts without their consent if they know the public key.
* Solution: Use `Signer<'info>`, which ensures that the account must be signed by the user's private key, making it impossible for unauthorized users to perform operations on behalf of others.
<br>

#### Exploiting Arbitrary CPI

* Verify that the target program to be invoked has the correct address.
* For example, if the main program invokes an external program to transfer funds from a user account to a pool account and the program does not verify the address of the external program, an arbitrary code execution can happen.
* To mitigate, replace the `AccountInfo<'info>` type (which is unverified) with Anchor's `Program<'info, T>` type.
* Note that Anchor supports `System`, `Token`, and `AssociatedToken` programs, but other programs must have the CPI modules generated.
* To learn more, check out [soldev.app's lesson on Arbitrary CPI](https://www.soldev.app/course/arbitrary-cpi).

<br>

#### Math & Logic Issues

* Beware of arithmetics and precision issues.
* Validate account data and instruction parameters.
* Make sure instructions are executed in the correct order.
* Make sure to prevent unintended behavior when passing duplicated accounts.
 
<br>

#### Reinitialization and Revival Attacks

* Make sure not to re-initialize an already-initialized account.
* Make sure to refrain from re-using an already closed account.
* To learn more, check out [soldev.app's lesson on Reinitialization Attacks](https://www.soldev.app/course/reinitialization-attacks).


<br>

#### PDAs

* Use canonical bump to avoid multiple valid PDAs (never let the user define an arbitrary bump).
* Do not share global PDA authorities; instead, use account-specific PDAs.
* To learn more, check out [soldev.app's lesson on Bump Seed Canonicalization](https://www.soldev.app/course/bump-seed-canonicalization).

<br>

---

## Security security resources

<br>