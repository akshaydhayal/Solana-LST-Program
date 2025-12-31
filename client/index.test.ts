import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, StakeProgram, SystemInstruction, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import {beforeAll, describe, it} from "bun:test";
import { LiteSVM } from "litesvm";

describe("stake manager tests",()=>{
    let svm:LiteSVM;
    let user:Keypair;
    let vote_acc:PublicKey;
    let stake_manager_prog:PublicKey;
    let connection:Connection;

    beforeAll(()=>{
        connection=new Connection(clusterApiUrl("devnet"),"confirmed");
        stake_manager_prog=new PublicKey("8xoE1aB6e3SpUMJ63RVaBEYEoLP4XGDPDtdySCRqP9Q8");
        user=Keypair.fromSecretKey(Uint8Array.from([48,182,182,234,169,224,236,113,52,199,47,66,39,2,163,52,183,44,45,27,127,49,133,151,64,70,248,16,46,218,234,198,42,180,5,68,243,235,189,56,197,37,17,85,205,189,100,191,64,74,171,3,37,193,199,195,213,54,156,198,228,15,248,188]));
        console.log("user : ",user.publicKey.toBase58());
        vote_acc=new PublicKey("he1iusunGwqrNtafDtLdhsUQDFvo13z9sUa36PauBtk");

        // svm=new LiteSVM();
        // user=Keypair.generate();
        // stake_manager_prog=PublicKey.unique();
        // vote_acc=new PublicKey("he1iusunGwqrNtafDtLdhsUQDFvo13z9sUa36PauBtk");

        // svm.airdrop(user.publicKey, BigInt(LAMPORTS_PER_SOL));
        // svm.addProgramFromFile(stake_manager_prog,"../target/deploy/stake_manager.so");

    }),
    it("test1",async()=>{
        let ix=new TransactionInstruction({
            programId:stake_manager_prog,
            keys:[
                {pubkey:user.publicKey, isSigner:true, isWritable:false},
                {pubkey:vote_acc, isSigner:false, isWritable:false},
                {pubkey:StakeProgram.programId, isSigner:false, isWritable:false}
            ],
            data:Buffer.from([])
        });
        let tx=new Transaction().add(ix);
        tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        tx.sign(user);
        let txStatus=await connection.sendRawTransaction(tx.serialize());
        console.log("txStatus : ",txStatus.toString());

        // tx.recentBlockhash=svm.latestBlockhash();
        // tx.sign(user);
        // let txStatus=svm.sendTransaction(tx);
        // console.log("txStatus : ",txStatus.toString());
    })
})