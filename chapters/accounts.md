# Accounts

### System Program

* The system program is a program built into the Solana runtime that transfers SOL from one account to another.

<br>

---

### 

* A program can own multiple accounts, it “discriminates” among the accounts with the “seed” which is used in calculating a “discriminator”. The “discriminator” takes up 8 bytes, which is why we need to allocate the additional 8 bytes in addition to the space our struct takes up. The bump can be treated as boilerplate for now.