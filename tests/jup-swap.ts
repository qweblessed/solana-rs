import { ProgramC } from '../target/types/program_c';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction
} from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import fs from "fs"; 

describe("program-C", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Jupiter swap aggregation", async () => {
    
  });
});

