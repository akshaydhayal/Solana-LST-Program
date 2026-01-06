import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import {beforeAll, describe, it} from "bun:test";
import * as spl from "@solana/spl-token";

describe("lst manager tests",()=>{
    let user:Keypair;
    let lst_manager_prog:PublicKey;
    
    let lst_manager_pda:PublicKey;
    let stake_manager_pda:PublicKey;
    let lst_manager_bump:number;
    let lst_manager_pda_vault:PublicKey;
    let lst_manager_vault_bump:number;
    
    // let lst_mint:Keypair;
    let lst_mint_pda:PublicKey;
    let lst_mint_pda_bump:number;

    let connection:Connection;
    beforeAll(async()=>{
        user=Keypair.fromSecretKey(Uint8Array.from([48,182,182,234,169,224,236,113,52,199,47,66,39,2,163,52,183,44,45,27,127,49,133,151,64,70,248,16,46,218,234,198,42,180,5,68,243,235,189,56,197,37,17,85,205,189,100,191,64,74,171,3,37,193,199,195,213,54,156,198,228,15,248,188]));
        lst_manager_prog=new PublicKey("9dCnABZLo3LQt4zy5mGZwADPZeK5JBTb32NsdC5KgzC6");
        stake_manager_pda=new PublicKey("5AMSMnbG9ZcV5LsuwQNfBGxtqeBzoRuhJcggajkyqnu8");
    
        connection=new Connection(clusterApiUrl("devnet"),"confirmed");

        [lst_manager_pda,lst_manager_bump]=PublicKey.findProgramAddressSync([Buffer.from("lst_manager")],lst_manager_prog);
        [lst_manager_pda_vault,lst_manager_vault_bump]=PublicKey.findProgramAddressSync([Buffer.from("lst_manager_vault"), lst_manager_pda.toBuffer()],lst_manager_prog);
        [lst_mint_pda,lst_mint_pda_bump]=PublicKey.findProgramAddressSync([Buffer.from("lst_mint"), lst_manager_pda.toBuffer()],lst_manager_prog);
    
        // async function createLSTMint(){
        //     lst_mint=Keypair.generate();
        //     let ix=SystemProgram.createAccount({
        //         fromPubkey:user.publicKey, newAccountPubkey:lst_mint.publicKey,
        //         lamports:await connection.getMinimumBalanceForRentExemption(spl.MINT_SIZE),
        //         space:spl.MINT_SIZE, programId:spl.TOKEN_PROGRAM_ID
        //     });
        //     let tx=new Transaction().add(ix);
        //     tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        //     tx.sign(user, lst_mint);
        //     let txStatus=await connection.sendRawTransaction(tx.serialize());
        //     await connection.confirmTransaction(txStatus,"confirmed");
        //     console.log("lst_mint account create txStatus : ",txStatus);
        // }
        // await createLSTMint();

        console.log("user : ",user.publicKey.toBase58());
        console.log("lst manager prog : ",lst_manager_prog.toBase58());
        console.log("lst manager pda : ",lst_manager_pda.toBase58());
        console.log("lst manager pda vault : ",lst_manager_pda_vault.toBase58());
        console.log("lst mint pda : ",lst_mint_pda.toBase58());
    })

    it("initialise lst manager",async()=>{
        let ix=new TransactionInstruction({
            programId:lst_manager_prog,
            keys:[
                {pubkey:user.publicKey, isSigner:true, isWritable:false},
                {pubkey:stake_manager_pda, isSigner:false, isWritable:true},
                {pubkey:lst_manager_pda, isSigner:false, isWritable:true},
                {pubkey:lst_manager_pda_vault, isSigner:false, isWritable:true},
                {pubkey:lst_mint_pda, isSigner:false, isWritable:true},
                {pubkey:SystemProgram.programId, isSigner:false, isWritable:false},
                {pubkey:spl.TOKEN_PROGRAM_ID, isSigner:false, isWritable:false}
            ],
            data:Buffer.concat([
                Buffer.from([0]),
                Buffer.from([lst_manager_bump]),
                Buffer.from([lst_manager_vault_bump]),
                Buffer.from([lst_mint_pda_bump]),
            ])
        });
        let tx=new Transaction().add(ix);
        tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        tx.sign(user);
        let txStatus=await connection.sendRawTransaction(tx.serialize());
        await connection.confirmTransaction(txStatus);
        console.log("initialise lst manager tx status : ",txStatus);
    })
})