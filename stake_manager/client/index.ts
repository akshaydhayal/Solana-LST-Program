import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";

let connection=new Connection(clusterApiUrl("devnet"),"confirmed");
async function getAccount(acc:PublicKey){
    let acc_data=await connection.getAccountInfo(acc);
    console.log(acc_data);
    console.log(acc_data?.data);
}

async function getVoteAccounts(){
    let voteAccounts=await connection.getVoteAccounts();
    console.log(voteAccounts.current);
    console.log("current validators len : ",voteAccounts.current.length);
    console.log("delinquent validators len : ",voteAccounts.delinquent.length);
}
// getVoteAccounts();
// getAccount(new PublicKey("DdjZ2TxAr56PTMpsuNzmuWSkexVrcNiCfBWiCCBpsb6A"));

let config_pubkey=new PublicKey([3, 6, 74, 163, 0, 47, 116, 220, 200, 110, 67, 49, 15, 12, 5, 42, 248, 197, 218, 39, 246, 16, 64, 25, 163, 35, 239, 160, 0, 0, 0, 0]);
console.log(config_pubkey.toBase58());

let stake_acc=new PublicKey([
    2, 4, 70, 141, 146, 173, 218, 246, 243, 43, 0, 50, 107, 3, 111, 233, 95, 247,
    87, 189, 84, 119, 103, 114, 23, 148, 19, 117, 237, 36, 120, 197
  ]);
console.log(stake_acc.toBase58());