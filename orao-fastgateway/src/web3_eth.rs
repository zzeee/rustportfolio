use web3::{contract::{Contract, Options}, types::{Address, H256, U256}, ethabi};
use futures::executor;//future::ok as fut_ok;
//use futures::future::FutureResult;
use std::str::FromStr;
use web3::contract::tokens::Tokenize;
use web3::types::{TransactionRequest, Bytes, TransactionParameters};
use web3::ethabi::Token;
use web3::futures::Future;
use secp256k1::SecretKey;
use crate::data::{BlockchainWriterLine, AddrType, OracleStatData};



pub async fn write_eth_event_wrapped(data:&BlockchainWriterLine) -> web3::Result<H256> {
     write_eth_event(&*data.contract_address, &*data.sec_key,data.u1,data.u2,data.u3,data.u4,data.u5,data.u6).await
}

pub async fn sendEth(addresses: &Vec<&AddrType>, orao_stat_data_cp: &Vec<OracleStatData>, seckey: String) {
    println!("SENDETH!");
    for contract_address in (addresses).iter() {
 for val in orao_stat_data_cp.iter() {
            // println!("Writing: {:?}", val);
            let key = val.key;
            let value = ((val.average as f64) * 1000.0) as i64;
            let baseVal = 1000;
            let timestamp = val.timestamp;
            let data_pool = BlockchainWriterLine {
                network_id: contract_address.network_id,
                contract_address: contract_address.address.clone(),
                sec_key: seckey.clone(),
                u1: 0,
                u2: 0,
                u3: key as i32,
                u4: value as i32,
                u5: baseVal,
                u6: timestamp as i32,
            };
     write_eth_event_wrapped(&data_pool).await;
           // let _ = service::write_event_queue(&plasm_events_queue, data_pool).await;
        }

    }

}


pub async fn write_eth_event(contract_address:&str,sec_key:&str, u1: i32, u2: i32, u3: i32, u4: i32, u5: i32, u6: i32) -> web3::Result<H256> {
    let addr= std::env::var("ETHEREUM_GATEWAY").unwrap_or("https://rinkeby.infura.io/v3/b568b550c01c4cbaab4e92e36a5aafba".to_string());
    let transport = web3::transports::Http::new(&*addr)?;
    let web3 = web3::Web3::new(transport);
     let seckey: secp256k1::key::SecretKey = sec_key.parse().unwrap();
     let contract = Contract::from_json(
        web3.eth(), Address::from_str(contract_address).unwrap(), include_bytes!("../solidity/Plasm_OraoInfo_sol_OraoInfo.abi"),
    ).unwrap();

    println!("Ethereum:{:?}", contract_address);

   // let data = contract.abi().function("check").unwrap().encode_input(&vec![Token::Int(U256::from(100))].into_tokens());
    let Options {
        gas,
        gas_price,
        value,
        nonce,
        condition,
    } = Options::default();

   // let my_account = Address::from_str("0x796A1027e18Bc0284d9A39C951cf56408A5cB825").unwrap();

    let data2 = contract.abi().function("addOraoInfo").unwrap().encode_input(&(U256::from(u1), U256::from(u2), U256::from(u3), U256::from(u4), U256::from(u5), U256::from(u6)).into_tokens()).unwrap();
    let TransactionParameters {
        gas,
        gas_price,
        value,
        nonce, ..
    } = TransactionParameters::default();


    let addr1=Address::from_str(contract_address).unwrap();
    let tx = TransactionParameters {
        to: Some(addr1),
        gas,
        value,
        gas_price,
        nonce,
        data: Bytes(data2),
        chain_id: None,
    };



    let signed = web3.accounts().sign_transaction(tx.clone(), &seckey).await;
    println!("writeEvent!!!! addr: {:?} sign {:?}",contract_address, signed);

    if let Ok(signed1)=signed {
        let result: web3::Result<H256> = web3.eth().send_raw_transaction(signed1.raw_transaction).await;
        println!("RESULT: {:?}", result);
        return result;
    }
    if let Err(nsugned)=signed {
        println!("RESULT nsugned: {:?}", nsugned);
        return Err(nsugned);

    }


    Ok(H256::from([0;32]))
}