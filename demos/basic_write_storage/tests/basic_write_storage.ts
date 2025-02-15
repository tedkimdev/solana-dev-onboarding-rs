import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicWriteStorage } from "../target/types/basic_write_storage";

describe("basic_write_storage", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicWriteStorage as Program<BasicWriteStorage>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      myStorage: myStorage,
      signer: program.provider.publicKey,
    }).rpc();
    console.log("Your transaction signature", tx);

    let myStorageStruct = await program.account.myStorage.fetch(myStorage);
    console.log("The value of x is:", myStorageStruct.x.toString());
  });

  it("set!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    const tx = await program.methods.set(new anchor.BN(3), new anchor.BN(4), new anchor.BN(5)).accounts({
      myStorage: myStorage,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("print x!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    const tx = await program.methods.printX().accounts({
      myStorage: myStorage,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
