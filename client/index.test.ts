import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, StakeProgram, SystemInstruction, SystemProgram, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY, SYSVAR_STAKE_HISTORY_PUBKEY, Transaction, TransactionInstruction } from "@solana/web3.js";
import {beforeAll, describe, it} from "bun:test";
import { LiteSVM } from "litesvm";
import * as borsh from "borsh";

let allowed_validators_schema:borsh.Schema={
    struct:{
        validators:{array:{
            type:{
                array:{type:'u8',len:32}
            }
        }}
    }
}

class allowedValidators{
    validators:PublicKey[];

    constructor(validators:PublicKey[]){
        this.validators=validators;
    }
}

let managerPdaSchema:borsh.Schema={
    struct:{
        admin:{array:{type:'u8',len:32}},
        total_staked:'u64',
        allowedValidators:{array:{
            type:{
                array:{type:'u8',len:32}
            }
        }},
    }
}
let userPositionPdaSchema:borsh.Schema={
    struct:{
        owner:{array:{type:'u8', len:32}},
        stake_acc:{array:{type:'u8', len:32}},
        deposited_amount:'u64'
    }
}

let stakeAmountSchema:borsh.Schema={
    struct:{
        stakeAmount:'u64'
    }
}

describe("stake manager tests",()=>{
    let connection:Connection;
    let user:Keypair;
    let user2:Keypair;
    let user3:Keypair;
    let user4:Keypair;
    let user5:Keypair;
    let user6:Keypair;
    let user7:Keypair;
    let user8:Keypair;
    let user9:Keypair;
    let user10:Keypair;

    let vote_acc:PublicKey;
    let stake_manager_prog:PublicKey;
    
    let stake_acc:Keypair;
    
    let stake_acc1:PublicKey;    // user1 stake account address
    let stake_acc2:PublicKey;    // user2 stake account address

    let manager_pda:PublicKey;
    let user_position_pda:PublicKey;
    let manager_bump:number;
    let user_position_bump:number;
    
    //local accounts
    // let svm:LiteSVM;
    // let user_local:Keypair;
    // let stake_acc_local:Keypair;
    // let stake_manager_prog_local:PublicKey;
    // let manager_pda_local:PublicKey;
    // let user_position_pda_local:PublicKey;
    // let manager_bump_local:number;
    // let user_position_bump_local:number;


    beforeAll(()=>{
        connection=new Connection(clusterApiUrl("devnet"),"confirmed");           
        stake_manager_prog=new PublicKey("HH9bAri2J7dev7CqSntxV9UxbyTjMVgxhqoHHKTttoBS");
        user=Keypair.fromSecretKey(Uint8Array.from([48,182,182,234,169,224,236,113,52,199,47,66,39,2,163,52,183,44,45,27,127,49,133,151,64,70,248,16,46,218,234,198,42,180,5,68,243,235,189,56,197,37,17,85,205,189,100,191,64,74,171,3,37,193,199,195,213,54,156,198,228,15,248,188]));
        user2=Keypair.fromSecretKey(Uint8Array.from([215,165,68,175,47,210,128,121,121,97,23,179,164,131,228,223,218,91,155,232,39,124,210,236,103,153,216,114,223,219,137,137,89,196,91,153,37,15,135,74,96,38,113,98,150,246,165,78,179,174,73,196,27,197,138,84,55,6,99,240,135,98,229,81]));
        user3=Keypair.fromSecretKey(Uint8Array.from([116,66,200,166,57,160,236,230,91,117,58,224,252,24,48,206,147,97,25,252,124,178,196,133,155,100,0,177,220,10,116,163,246,134,161,240,247,1,147,224,162,181,202,226,124,167,163,142,55,161,226,65,56,153,46,143,24,86,209,239,55,104,26,133]));
        user4=Keypair.fromSecretKey(Uint8Array.from([63,230,58,57,192,92,197,152,80,80,20,205,70,143,173,122,201,193,177,18,97,244,27,216,36,98,70,85,118,61,126,172,90,121,0,226,122,11,36,77,14,226,64,157,89,253,125,17,10,154,210,223,219,56,8,28,112,16,206,112,252,201,243,216]));
        user5=Keypair.fromSecretKey(Uint8Array.from([251,194,223,127,208,99,213,193,41,77,87,176,238,70,163,7,233,53,222,111,197,213,208,77,36,186,223,127,111,115,1,6,178,155,232,205,62,117,48,214,199,85,123,4,42,141,99,130,81,225,228,252,165,37,187,113,88,7,30,63,82,181,127,195]));
        user6=Keypair.fromSecretKey(Uint8Array.from([32,107,118,195,118,32,253,173,222,59,72,14,72,185,221,196,65,3,178,222,76,58,227,45,193,93,97,130,84,133,79,149,254,81,69,163,57,38,252,116,134,227,92,75,157,236,130,93,229,138,14,230,226,174,142,100,143,159,95,129,236,222,171,100]));
        user7=Keypair.fromSecretKey(Uint8Array.from([101,254,214,57,203,98,77,182,60,116,192,105,201,142,231,252,12,150,187,57,78,198,6,9,214,13,208,197,116,24,8,17,16,62,119,102,195,227,38,222,191,44,83,162,8,222,92,107,207,6,248,190,175,70,39,250,124,104,69,12,106,240,36,226]));
        user8=Keypair.fromSecretKey(Uint8Array.from([127,46,141,69,134,123,231,42,222,30,95,167,148,45,193,234,158,173,17,80,46,198,28,86,135,117,203,37,231,103,146,35,130,231,239,73,208,5,75,160,122,232,126,41,167,27,131,53,51,226,2,78,85,193,212,166,56,234,73,185,217,128,177,192]));
        user9=Keypair.fromSecretKey(Uint8Array.from([93,134,72,57,18,217,43,143,68,107,34,81,51,47,209,134,197,115,167,155,131,169,152,178,146,79,215,211,42,137,176,162,2,217,103,144,6,252,172,248,181,107,209,8,89,249,29,189,227,127,95,228,68,45,9,110,46,151,121,83,15,218,240,131]));
        user10=Keypair.fromSecretKey(Uint8Array.from([119,173,62,21,247,119,229,25,16,142,204,244,138,255,216,212,141,16,86,177,131,176,205,209,255,213,20,176,202,78,158,171,96,162,191,64,224,118,95,210,181,170,141,251,131,185,96,89,42,114,235,231,123,160,206,12,80,223,105,58,219,116,78,130]));
       
        // vote_acc=new PublicKey("he1iusunGwqrNtafDtLdhsUQDFvo13z9sUa36PauBtk");
        vote_acc=new PublicKey("23AoPQc3EPkfLWb14cKiWNahh1H9rtb3UBk8gWseohjF");
        
        // [manager_pda,manager_bump]=PublicKey.findProgramAddressSync([Buffer.from("manager"), user.publicKey.toBuffer()], stake_manager_prog);
        [manager_pda,manager_bump]=PublicKey.findProgramAddressSync([Buffer.from("manager")], stake_manager_prog);
        [user_position_pda,user_position_bump]=PublicKey.findProgramAddressSync([Buffer.from("position"), user.publicKey.toBuffer()], stake_manager_prog);
        // [user_position_pda,user_position_bump]=PublicKey.findProgramAddressSync([Buffer.from("position"), user.publicKey.toBuffer()], stake_manager_prog);
        
        stake_acc=Keypair.generate();
        stake_acc1=new PublicKey("8sbcVDyjLgvfPRkSLD1H3sGw6BR3fG42xTEk5Hc35rU");
        stake_acc2=new PublicKey("BZXHZXyGyu2L8iar6z4YfthLMvDhXcbmUnASsMJVBhnm");

        console.log("user : ",user.publicKey.toBase58());
        console.log("user2 : ",user2.publicKey.toBase58());
        console.log("stake_manager_prog : ",stake_manager_prog.toBase58());
        console.log("manager_pda : ",manager_pda.toBase58()); 
        console.log("user_position_pda : ",user_position_pda.toBase58()); 
        console.log("user2 stake_acc : ",stake_acc.publicKey.toBase58());
        
        //local testing
        // svm=new LiteSVM();
        // user_local=Keypair.generate();
        // stake_manager_prog_local=PublicKey.unique();
        // stake_acc_local=Keypair.generate();

        // svm.airdrop(user_local.publicKey, BigInt(LAMPORTS_PER_SOL));
        // svm.addProgramFromFile(stake_manager_prog_local,"../target/deploy/stake_manager.so");

        // [manager_pda_local,manager_bump_local]=PublicKey.findProgramAddressSync([Buffer.from("manager"), user_local.publicKey.toBuffer()], stake_manager_prog_local);
        // [user_position_pda_local,user_position_bump_local]=PublicKey.findProgramAddressSync([Buffer.from("position"), user_local.publicKey.toBuffer()], stake_manager_prog_local);
        
        // console.log("user_local : ",user_local.publicKey.toBase58());
        // console.log("stake_manager_prog_local : ",stake_manager_prog_local.toBase58());
        // console.log("manager_pda_local : ",manager_pda_local.toBase58());
        // console.log("user_position_pda_local : ",user_position_pda_local.toBase58());
        // console.log("stake_acc_local : ",stake_acc_local.publicKey.toBase58());

    }),

    // it("create manager test",async()=>{
    //     console.log("devnet create manager test")
    //     let allowed_validators=[vote_acc.toBytes(), new PublicKey("votem3UdGx5xWFbY9EFbyZ1X2pBuswfR5yd2oB3JAaj").toBytes()];
    //     // let serialised_allowed_validators=borsh.serialize(allowed_validators_schema, new allowedValidators(allowed_validators));
    //     let serialised_allowed_validators=borsh.serialize(allowed_validators_schema, {validators:allowed_validators});
    //     console.log("serialised_allowed_validators : ",serialised_allowed_validators);

    //     let ix=new TransactionInstruction({
    //         programId:stake_manager_prog,
    //         keys:[
    //             {pubkey:user.publicKey, isSigner:true, isWritable:false},
    //             {pubkey:manager_pda, isSigner:false, isWritable:true},
    //             {pubkey:SystemProgram.programId, isSigner:false, isWritable:false}
    //         ],
    //         data:Buffer.concat([
    //             Buffer.from([0]),
    //             // Buffer.from([0,0,0,0]),
    //             Buffer.from(serialised_allowed_validators),
    //             Buffer.from([manager_bump])
    //         ])
    //     });
    //     let tx=new Transaction().add(ix);
    //     tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
    //     tx.sign(user);
    //     let txStatus=await connection.sendRawTransaction(tx.serialize());
    //     await connection.confirmTransaction(txStatus,"confirmed");
    //     console.log("create manager txStatus : ",txStatus.toString());

    //     let manager_pda_data=await connection.getAccountInfo(manager_pda,"confirmed");
    //     console.log("manager_pda_data : ", manager_pda_data?.data);  
    //     console.log("manager_pda_data : ",borsh.deserialize(managerPdaSchema,manager_pda_data?.data));
    //     //litesvm_testing
    //     // console.log("local create manager test")
    //     // let ix_local=new TransactionInstruction({
    //     //     programId:stake_manager_prog_local,
    //     //     keys:[
    //     //         {pubkey:user_local.publicKey, isSigner:true, isWritable:false},
    //     //         {pubkey:manager_pda_local, isSigner:false, isWritable:true},
    //     //         {pubkey:SystemProgram.programId, isSigner:false, isWritable:false}
    //     //     ],
    //     //     data:Buffer.concat([
    //     //         Buffer.from([0]),
    //     //         Buffer.from([0,0,0,0]),
    //     //         Buffer.from([manager_bump_local])
    //     //     ])
    //     // });
    //     // let tx_local=new Transaction().add(ix_local);
    //     // tx_local.recentBlockhash=svm.latestBlockhash();
    //     // tx_local.sign(user_local);
    //     // let txStatus_local=svm.sendTransaction(tx_local);
    //     // console.log("create manager txStatus_local : ",txStatus_local.toString());
    // }),
   
    
    // it("create stake account test",async()=>{
    //     console.log("devnet create stake acc test");
    //     let serialised_stake_amount=borsh.serialize(stakeAmountSchema,{stakeAmount:0.8*LAMPORTS_PER_SOL});
    //     console.log("serialised_stake_amount : ",serialised_stake_amount);
    //     let ix=new TransactionInstruction({
    //         programId:stake_manager_prog,
    //         keys:[
    //             {pubkey:user10.publicKey, isSigner:true, isWritable:false},
    //             {pubkey:manager_pda, isSigner:false, isWritable:true},
    //             {pubkey:user_position_pda, isSigner:false, isWritable:true},
    //             {pubkey:stake_acc.publicKey, isSigner:true, isWritable:true},
    //             {pubkey:SystemProgram.programId, isSigner:false, isWritable:false},
    //             {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
    //             {pubkey:SYSVAR_RENT_PUBKEY, isSigner:false, isWritable:false},
    //             {pubkey:SystemProgram.programId, isSigner:false, isWritable:false}
    //         ],
    //         data:Buffer.concat([
    //             Buffer.from([1]),
    //             Buffer.from(serialised_stake_amount),
    //             // Buffer.from([99,0,0,0,0,0,0,0]),
    //             Buffer.from([manager_bump]),
    //             Buffer.from([user_position_bump])
    //         ])
    //     });
    //     let tx=new Transaction().add(ix);
    //     tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
    //     tx.sign(user10,stake_acc);
    //     let txStatus=await connection.sendRawTransaction(tx.serialize());
    //     await connection.confirmTransaction(txStatus,"confirmed");
    //     console.log("create stake acc txStatus : ",txStatus.toString());
        
    //     let stake_acc_data=await connection.getAccountInfo(stake_acc.publicKey,"confirmed");
    //     console.log("stake_acc_data : ",stake_acc_data);

    //     let manager_pda_data=await connection.getAccountInfo(manager_pda);
    //     console.log("manager_pda_data : ", borsh.deserialize(managerPdaSchema,manager_pda_data?.data));

    //     let user_position_pda_data=await connection.getAccountInfo(user_position_pda,"confirmed");
    //     console.log("user_position_pda_data : ",user_position_pda_data);
        
    //     //local litesvm tests
    //     // console.log("local create stake acc test")
    //     // let ix_local=new TransactionInstruction({
    //     //     programId:stake_manager_prog_local,
    //     //     keys:[
    //     //         {pubkey:user_local.publicKey, isSigner:true, isWritable:false},
    //     //         {pubkey:manager_pda_local, isSigner:false, isWritable:true},
    //     //         {pubkey:stake_acc_local.publicKey, isSigner:true, isWritable:true},
    //     //         {pubkey:SystemProgram.programId, isSigner:false, isWritable:false},
    //     //         {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
    //     //         {pubkey:SYSVAR_RENT_PUBKEY, isSigner:false, isWritable:false}
    //     //     ],
    //     //     data:Buffer.concat([
    //     //         Buffer.from([1]),
    //     //         Buffer.from([99,0,0,0,0,0,0,0]),
    //     //         Buffer.from([manager_bump_local])
    //     //     ])
    //     // });
    //     // let tx_local=new Transaction().add(ix_local);
    //     // tx_local.recentBlockhash=svm.latestBlockhash();
    //     // tx_local.sign(user_local, stake_acc_local);
    //     // let txStatus_local=svm.sendTransaction(tx_local);
    //     // console.log("create stake acc txStatus_local : ",txStatus_local.toString());

    //     // let stake_acc_local_data=svm.getAccount(stake_acc_local.publicKey);
    //     // console.log("stake_acc_local_data : ",stake_acc_local_data);
    // }),


    // it("delegate stake to validator test", async()=>{
    //     const STAKE_CONFIG_ID = new PublicKey("StakeConfig11111111111111111111111111111111");
    //     let ix=new TransactionInstruction({
    //         programId:stake_manager_prog,
    //         keys:[
    //             {pubkey:user10.publicKey, isSigner:true, isWritable:false},
    //             {pubkey:manager_pda, isSigner:false, isWritable:false},
    //             {pubkey:stake_acc.publicKey, isSigner:false, isWritable:true}, //needs to be signer and maybe writable also
    //             {pubkey:vote_acc, isSigner:false, isWritable:false},
    //             {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
    //             {pubkey:SYSVAR_CLOCK_PUBKEY, isSigner:false, isWritable:false},
    //             {pubkey:SYSVAR_STAKE_HISTORY_PUBKEY, isSigner:false, isWritable:false},
    //             {pubkey:STAKE_CONFIG_ID, isSigner:false, isWritable:false},
    //         ],
    //         data:Buffer.concat([
    //             Buffer.from([2]),
    //             Buffer.from([manager_bump])
    //         ])
    //     });
    //     let tx=new Transaction().add(ix);
    //     tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
    //     tx.sign(user10);
    //     let txStatus=await connection.sendRawTransaction(tx.serialize());
    //     await connection.confirmTransaction(txStatus); 
    //     console.log("delegate stake tx : ",txStatus);
    // }),


    // it("deactivate stake test", async()=>{
    //     let ix=new TransactionInstruction({
    //         programId:stake_manager_prog,
    //         keys:[
    //             {pubkey:user.publicKey, isSigner:true, isWritable:false},
    //             {pubkey:manager_pda, isSigner:false, isWritable:false},
    //             {pubkey:stake_acc1, isSigner:false, isWritable:true}, //needs to be signer and maybe writable also
    //             {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
    //             {pubkey:SYSVAR_CLOCK_PUBKEY, isSigner:false, isWritable:false},
    //         ],
    //         data:Buffer.concat([
    //             Buffer.from([3]),
    //             Buffer.from([manager_bump])
    //         ])
    //     });
    //     let tx=new Transaction().add(ix);
    //     tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
    //     tx.sign(user);
        
    //     let txStatus=await connection.sendRawTransaction(tx.serialize());
    //     await connection.confirmTransaction(txStatus); 
    //     console.log("dectivate stake tx : ",txStatus);

    //     let manager_pda_data=await connection.getAccountInfo(manager_pda);
    //     console.log("manager_pda_data : ", borsh.deserialize(managerPdaSchema, manager_pda_data?.data));
        
    //     let user_position_pda_data=await connection.getAccountInfo(user_position_pda);
    //     console.log(" user_position_pda_data : ", borsh.deserialize(userPositionPdaSchema, user_position_pda_data?.data));
    // }),

    it("withdraw stake test", async()=>{
        let ix=new TransactionInstruction({
            programId:stake_manager_prog,
            keys:[
                {pubkey:user.publicKey, isSigner:true, isWritable:false},
                {pubkey:manager_pda, isSigner:false, isWritable:false},
                {pubkey:user_position_pda, isSigner:false, isWritable:false},
                {pubkey:stake_acc1, isSigner:false, isWritable:true}, //needs to be signer and maybe writable also
                {pubkey:StakeProgram.programId, isSigner:false, isWritable:false},
                {pubkey:SYSVAR_CLOCK_PUBKEY, isSigner:false, isWritable:false},
                {pubkey:SYSVAR_STAKE_HISTORY_PUBKEY, isSigner:false, isWritable:false},
            ],
            data:Buffer.concat([
                Buffer.from([4]),
                Buffer.from([manager_bump]),
                Buffer.from([user_position_bump])
            ])
        });
        let tx=new Transaction().add(ix);
        tx.recentBlockhash=(await connection.getLatestBlockhash()).blockhash;
        tx.sign(user);
        
        let txStatus=await connection.sendRawTransaction(tx.serialize());
        await connection.confirmTransaction(txStatus); 
        console.log("withdraw stake tx : ",txStatus);

        let manager_pda_data=await connection.getAccountInfo(manager_pda);
        console.log("manager_pda_data : ", borsh.deserialize(managerPdaSchema, manager_pda_data?.data));
        
        let user_position_pda_data=await connection.getAccountInfo(user_position_pda);
        console.log(" user_position_pda_data : ", borsh.deserialize(userPositionPdaSchema, user_position_pda_data?.data));
    })
})