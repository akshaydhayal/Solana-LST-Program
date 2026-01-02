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
getVoteAccounts();
// getAccount(new PublicKey("DdjZ2TxAr56PTMpsuNzmuWSkexVrcNiCfBWiCCBpsb6A"));