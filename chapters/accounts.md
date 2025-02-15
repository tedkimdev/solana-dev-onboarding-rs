# Accounts

### System Program

* The system program is a program built into the Solana runtime that transfers SOL from one account to another.

<br>

---

### 

* A program can own multiple accounts, it “discriminates” among the accounts with the “seed” which is used in calculating a “discriminator”. The “discriminator” takes up 8 bytes, which is why we need to allocate the additional 8 bytes in addition to the space our struct takes up. The bump can be treated as boilerplate for now.


<br>

---

#### UncheckedAccount

* The `UncheckedAccount` type tells to Anchor to not check if the account being read is owned by the program.

* Note that the account we passed through the Context struct is not an account that a program initialized, hence the program does not own it.

* When Anchor reads an account of type Account in the `#[derive(Accounts)]`, it will check (behind the scenes) if that account is owned by that program. If not, the execution will halt.

* If a malicious user crafts an account the program did not create and then passes it to the Solana program, and the Solana program blindly trusts the data in the account, critical errors may occur.

* The Anchor framework checks behind the scenes to see if the account is not owned by the program, and rejects reading the account. `UncheckedAccount` bypasses this safety check.

* ** `AccountInfo` and `UncheckedAccount` are aliases for each other and `AccountInfo` has the same security considerations.

* Without `/// Check:` comment and run `anchor build` you should see the build halt and ask you to add the comment back with an explanation for why an Unchecked Account is safe. 

* When we do not want to deserialize the data, we don’t supply an `#[account]` struct.
