use web3::{contract::{Contract, Options}, types::{Address, H256, U256}, ethabi};
use futures::executor;
//future::ok as fut_ok;
use std::str::FromStr;
use web3::contract::tokens::Tokenize;
use web3::types::{TransactionRequest, Bytes, TransactionParameters};
use web3::ethabi::Token;
use web3::futures::Future;
use secp256k1::SecretKey;

pub async fn write_event(contract_address:&str,sec_key:&str, u1: U256, u2: U256, u3: U256, u4: U256, u5: U256) -> web3::Result<H256> {
    let transport = web3::transports::Http::new("https://rpc.dusty.plasmnet.io:8545")?;
    let web3 = web3::Web3::new(transport);
     let seckey: secp256k1::key::SecretKey = sec_key.parse().unwrap();
    let contract = web3::Contract::from_json(
        web3.eth(), Address::from_str(contract_address).unwrap(), include_bytes!("../solidity/ProviderInfo_sol_ProviderInfo.abi"),
    ).unwrap();

    println!("writeEvent");

   // let data = contract.abi().function("check").unwrap().encode_input(&vec![Token::Int(U256::from(100))].into_tokens());
    let Options {
        gas,
        gas_price,
        value,
        nonce,
        condition,
    } = Options::default();

    let my_account = web3::Address::from_str("0x796A1027e18Bc0284d9A39C951cf56408A5cB825").unwrap();

    let data2 = contract.abi().function("addProviderInfo").unwrap().encode_input(&(U256::from(u1), U256::from(u2), U256::from(u3), U256::from(u4), U256::from(u5)).into_tokens()).unwrap();
    let TransactionParameters {
        gas,
        gas_price,
        value,
        nonce, ..
    } = TransactionParameters::default();


    let addr1=web3::Address::from_str(contract_address).unwrap();
    let tx = TransactionParameters {
        to: Some(addr1),
        gas,
        value,
        gas_price,
        nonce,
        data: Bytes(data2),
        chain_id: None,
    };


    let signed = futures::executor::block_on(web3.accounts().sign_transaction(tx.clone(), &seckey)).unwrap();
    let result:web3::Result<H256> = web3.eth().send_raw_transaction(signed.raw_transaction).await;
//    println!(" {:?}", rd);


    result
}