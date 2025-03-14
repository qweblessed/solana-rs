// import { ProgramC } from './../target/types/program_c';
// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";

// describe("program-C", () => {
//   // Set provider for Anchor tests
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const programC = anchor.workspace.ProgramC as Program<ProgramC>;
//   let signer = anchor.web3.Keypair.generate();

//   it("Creates a pool and deposits funds to the PDA!", async () => {
//     // Derive PDA address using pool address and signer
//     let poolAccount = anchor.web3.Keypair.generate();
//     let [pdaAddress, bump] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from('pool_pda'), poolAccount.publicKey.toBuffer(), signer.publicKey.toBuffer()],
//       programC.programId
//     );

//     // Airdrop SOL to signer account for testing purposes
//     await airdrop(programC.provider.connection, signer.publicKey, 500_000_000_000);

//     // Create the pool using the create_and_join_pool method
//     const tx = await programC.methods
//       .createAndJoinPool()  // Call the method that creates the pool
//       .accounts({
//         pool: poolAccount.publicKey,
//         pdaAccount: pdaAddress,
//         signer: signer.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([signer, poolAccount])
//       .rpc();

//     console.log("Pool creation and deposit transaction signature:", tx);

//     // After pool creation, test if the funds have been transferred to the PDA
//     const pdaBalance = await programC.provider.connection.getAccountInfo(pdaAddress);
//     console.log("PDA Account Balance:", pdaBalance?.lamports);


//   });
// });

// // Helper function to airdrop SOL to the signer's account
// export async function airdrop(connection: any, address: any, amount = 500_000_000_000) {
//   await connection.confirmTransaction(
//     await connection.requestAirdrop(address, amount),
//     'confirmed'
//   );
// }
