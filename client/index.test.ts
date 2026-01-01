import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, StakeProgram, SystemInstruction, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import {beforeAll, describe, it} from "bun:test";
import { LiteSVM } from "litesvm";

describe("stake manager tests",()=>{
    let svm:LiteSVM;
    let user:Keypair;
    let vote_acc:PublicKey;
    let stake_manager_prog:PublicKey;
    
    let stake_acc:Keypair;
    let manager_pda:PublicKey;
    let manager_bump:number;
    
    //local accounts
    let user_local:Keypair;
    let stake_acc_local:Keypair;
    let stake_manager_prog_local:PublicKey;
    let manager_pda_local:PublicKey;
    let manager_bump_local:number;

    let connection:Connection;

    beforeAll(()=>{
        connection=new Connection(clusterApiUrl("devnet"),"confirmed");
        stake_manager_prog=new PublicKey("8oDKRv6zAY25BzEiSNBzqHV6z8aAsofWSxBM1KAuVR9e");
        user=Keypair.fromSecretKey(Uint8Array.from([48,182,182,234,169,224,236,113,52,199,47,66,39,2,163,52,183,44,45,27,127,49,133,151,64,70,248,16,46,218,234,198,42,180,5,68,243,235,189,56,197,37,17,85,205,189,100,191,64,74,171,3,37,193,199,195,213,54,156,198,228,15,248,188]));
        vote_acc=new PublicKey("he1iusunGwqrNtafDtLdhsUQDFvo13z9sUa36PauBtk");
        
        [manager_pda,manager_bump]=PublicKey.findProgramAddressSync([Buffer.from("manager"), user.publicKey.toBuffer()], stake_manager_prog);
        stake_acc=Keypair.generate();

        console.log("user : ",user.publicKey.toBase58());
        console.log("stake_manager_prog : ",stake_manager_prog.toBase58());
        console.log("manager_pda : ",manager_pda.toBase58());
        console.log("stake_acc : ",stake_acc.publicKey.toBase58());
        
        
        
        //local testing
        svm=new LiteSVM();
        user_local=Keypair.generate();
        stake_manager_prog_local=PublicKey.unique();
        stake_acc_local=Keypair.generate();

        svm.airdrop(user_local.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.addProgramFromFile(stake_manager_prog_local,"../target/deploy/stake_manager.so");

        [manager_pda_local,manager_bump_local]=PublicKey.findProgramAddressSync([Buffer.from("manager"), user_local.publicKey.toBuffer()], stake_manager_prog_local);
        
        console.log("user_local : ",user_local.publicKey.toBase58());
        console.log("stake_manager_prog_local : ",stake_manager_prog_local.toBase58());
        console.log("manager_pda_local : ",manager_pda_local.toBase58());
        console.log("stake_acc_local : ",stake_acc_local.publicKey.toBase58());

    }),
    it("create manager test",async()=>{
        console.log("devnet create manager test")
        let ix=new TransactionInstruction({
            programId:stake_manager_prog,
            keys:[
                {pubkey:user.publicKey, isSigner:true, isWritable:false},
                {pubkey:manager_pda, isSigner:false, isWritable:true},
                {pubkey:SystemProgram.programId, isSigner:false, isWritable:false}
            ],
            data:Buffer.concat([
                Buffer.from([0]),
                Buffer.from([0,0,0,0]),
                Buffer.from([manager_bump])
            ])
        });
        let tx=new Transaction().add(ix);
        tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        tx.sign(user);
        let txStatus=await connection.sendRawTransaction(tx.serialize());
        console.log("create manager txStatus : ",txStatus.toString());

        //litesvm_testing
        console.log("local create manager test")
        let ix_local=new TransactionInstruction({
            programId:stake_manager_prog_local,
            keys:[
                {pubkey:user_local.publicKey, isSigner:true, isWritable:false},
                {pubkey:manager_pda_local, isSigner:false, isWritable:true},
                {pubkey:SystemProgram.programId, isSigner:false, isWritable:false}
            ],
            data:Buffer.concat([
                Buffer.from([0]),
                Buffer.from([0,0,0,0]),
                Buffer.from([manager_bump_local])
            ])
        });
        let tx_local=new Transaction().add(ix_local);
        tx_local.recentBlockhash=svm.latestBlockhash();
        tx_local.sign(user_local);
        let txStatus_local=svm.sendTransaction(tx_local);
        console.log("create manager txStatus_local : ",txStatus_local.toString());
    }),
    
    it("create stake account test",async()=>{
        console.log("devnet create stake acc test")
        let ix=new TransactionInstruction({
            programId:stake_manager_prog,
            keys:[
                {pubkey:user.publicKey, isSigner:true, isWritable:false},
                {pubkey:manager_pda, isSigner:false, isWritable:true},
                {pubkey:stake_acc.publicKey, isSigner:true, isWritable:true},
                {pubkey:SystemProgram.programId, isSigner:false, isWritable:false},
                {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
                {pubkey:SYSVAR_RENT_PUBKEY, isSigner:false, isWritable:false}
            ],
            data:Buffer.concat([
                Buffer.from([1]),
                Buffer.from([99,0,0,0,0,0,0,0]),
                Buffer.from([manager_bump])
            ])
        });
        let tx=new Transaction().add(ix);
        tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        tx.sign(user,stake_acc);
        let txStatus=await connection.sendRawTransaction(tx.serialize());
        console.log("create stake acc txStatus : ",txStatus.toString());
        
        let stake_acc_data=await connection.getAccountInfo(stake_acc_local.publicKey,"confirmed");
        console.log("stake_acc_data : ",stake_acc_data);

        //local litesvm tests
        console.log("local create stake acc test")
        let ix_local=new TransactionInstruction({
            programId:stake_manager_prog_local,
            keys:[
                {pubkey:user_local.publicKey, isSigner:true, isWritable:false},
                {pubkey:manager_pda_local, isSigner:false, isWritable:true},
                {pubkey:stake_acc_local.publicKey, isSigner:true, isWritable:true},
                {pubkey:SystemProgram.programId, isSigner:false, isWritable:false},
                {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
                {pubkey:SYSVAR_RENT_PUBKEY, isSigner:false, isWritable:false}
            ],
            data:Buffer.concat([
                Buffer.from([1]),
                Buffer.from([99,0,0,0,0,0,0,0]),
                Buffer.from([manager_bump_local])
            ])
        });
        let tx_local=new Transaction().add(ix_local);
        tx_local.recentBlockhash=svm.latestBlockhash();
        tx_local.sign(user_local, stake_acc_local);
        let txStatus_local=svm.sendTransaction(tx_local);
        console.log("create stake acc txStatus_local : ",txStatus_local.toString());

        let stake_acc_local_data=svm.getAccount(stake_acc_local.publicKey);
        console.log("stake_acc_local_data : ",stake_acc_local_data);
    })
})