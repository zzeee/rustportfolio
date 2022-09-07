use serde_derive::{Deserialize, Serialize};
use std::fmt;
use serde_json::json;
use std::sync::{Arc};
// use parking_lot::RwLock;
use std::collections::HashMap;
use std::vec::{Vec};
use tokio::sync::{MutexGuard, oneshot, Mutex as TMutex};
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use warp::{Rejection};

big_array! { BigArray; }


pub type Items = HashMap<String, i32>;
pub type OraoDataLines2 = HashMap<String, OraoLine>;
pub type OraoDataLines = Vec<OraoLine>;
pub type OraoDataLinesV3 = Vec<OraoLineV3>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;
pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type Result<T> = std::result::Result<T, Rejection>;

#[derive(Clone, Debug)]
pub struct OraoStoreS {
    pub requests: Vec<OraoArrayTimestamp>,
    pub counter: u32,
}

#[derive(Clone, Debug)]
pub struct OraoStoreSV3 {
    pub requests: Vec<OraoArrayTimestampV3>,
    pub counter: u32,
}

#[derive(Clone, Debug)]
pub struct BlockchainWriterLine {
    pub network_id: i32,
    pub contract_address: String,
    pub sec_key: String,
    pub u1: i32,
    pub u2: i32,
    pub u3: i32,
    pub u4: i32,
    pub u5: i32,
    pub u6: i32,
}

impl BlockchainWriterLine {
    pub fn new() -> Self {
        BlockchainWriterLine {
            network_id: 0,
            contract_address: ("".to_string()),
            sec_key: ("".to_string()),
            u1: 0,
            u2: 0,
            u3: 0,
            u4: 0,
            u5: 0,
            u6: 0,
        }
    }
}



#[derive(Debug)]
pub struct AddrType {
       pub network_id:i32,
       pub address:String
    }

pub type BlockchainWriterStore = Vec<BlockchainWriterLine>;

impl OraoStoreS {
    pub fn new() -> Self {
        OraoStoreS {
            requests: vec!(OraoArray_new()),
            counter: 0,
        }
    }
}

impl OraoStoreSV3 {
    pub fn new() -> Self {
        OraoStoreSV3 {
            requests: vec!(OraoArray_newV3()),
            counter: 0,
        }
    }
}
pub type BlockchainWriterMutex = Arc<TMutex<Vec<BlockchainWriterLine>>>;

pub type OraoStore = Arc<TMutex<OraoStoreS>>;
pub type OraoStoreV3 = Arc<TMutex<OraoStoreSV3>>;

#[derive(Clone, Deserialize, Debug, Serialize, Default)]
pub struct OraoLine {
    pub value_id: Vec<u8>,
    pub vector_id: u32,
    pub protocol_id: u32,
    pub value: f64,

    // confirmed:Vec<u8>
}


#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OraoExportLine {
    pub vector_id: u32,
    pub protocol_id: u32,
    pub value: u64,
    pub valuebase: u32,
    //  #[serde(with = "BigArray")]
    // pub provider: [u8; 64],
    pub provider: u64,
    //  pub provider_id:AccountId32
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OraoExportLineV3 {
    pub vector_id: u32,
    #[serde(with = "BigArray")]

    pub protocol_id: [u8; 64],
    pub value: u64,
    pub valuebase: u32,
    #[serde(with = "BigArray")]
    pub provider: [u8; 64],
    // pub provider: u64,
    //  pub provider_id:AccountId32
}


impl fmt::Display for OraoLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ORAOLINE: V:{} P:{} VAL:{} ", self.vector_id, self.protocol_id, self.value)
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct OraoArray {
    pub data: OraoDataLines,
    #[serde(with = "BigArray")]
    pub provider: [u8; 64],
}

//structure for adding data version 2 (no value_id)
#[derive(Clone, Deserialize, Debug, Serialize, Default)]
pub struct OraoLineV2 {
    pub vector_id: u32,
    pub protocol_id: u32,
    pub value: f64,
}

//structure for adding data version 2 (no value_id)
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OraoLineV3 {
    pub vector_id: u32,
    #[serde(with = "BigArray")]
    pub protocol_id: [u8; 64],
    pub value: f64,
}

pub struct calcDev {
    pub result: Vec::<f64>,
    pub total: f64,
    pub min: f64,
    pub max: f64,
    pub cnt: f64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct OraoArrayV2 {
    pub data: Vec<OraoLineV2>,
    #[serde(with = "BigArray")]
    pub provider: [u8; 64],
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct OraoArrayV3 {
    pub data: Vec<OraoLineV3>,
    #[serde(with = "BigArray")]
    pub provider: [u8; 64],
}
//end structure for adding data version 2 (no value_id)

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct OraoArrayTimestamp {
    pub data: OraoDataLines,
    pub timestamp: u64,
    pub provider: u64,
    //#[serde(with = "BigArray")]
    //pub provider: [u8; 64],

    // pub provider: Vec<u8>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct OraoArrayTimestampV3 {
    pub data: OraoDataLinesV3,
    pub timestamp: u64,
    #[serde(with = "BigArray")]
    pub provider: [u8; 64],

    // pub provider: Vec<u8>,
}

#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct ReqStruc {
    pub vector_id: u32,
    pub protocol_id: u32,
    pub node_id: u32,
    pub block_id: u32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ReqStrucV3 {
    pub vector_id: u32,
    #[serde(with = "BigArray")]
    pub protocol_id: [u8; 64],
    pub node_id: u32,
    pub block_id: u32,
}


#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ReqArrayStruc {
    pub vectors: Vec<u32>,
    #[serde(with = "BigArray")]
    pub protocol_id: [u8; 64],
    pub node_id: u32,
    pub block_id: u32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ReqArrayStrucV3 {
    pub vectors: Vec<u32>,
    #[serde(with = "BigArray")]
    pub protocol_id: [u8; 64],
    pub node_id: u32,
    pub block_id: u32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ReqDirect {
    //    #[serde(with = "BigArray")]
    pub node_id: u32,
    pub block_id: u32,
}


impl ReqDirect {
    pub fn new() -> Self {
        ReqDirect {
            node_id: 0,
            block_id: 0,
        }
    }
}

// structure used to keep in ram the key/value pairs of the Oracle data
#[derive(Clone, Debug)]
pub struct OracleData {
    pub key: u32,
    pub value: f64,
    pub timestamp: u64,
}

impl OracleData {
    pub fn new() -> Self {
        OracleData {
            key: 0,
            value: 0.0,
            timestamp: 0,
        }
    }
}

// structure to require average of Oracle Data
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct ReqAverage {
    pub vector_id: u32,
}

// structure to answer for average
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct AverageAnswer {
    pub key: u32,
    pub average: f64,
}

impl AverageAnswer {
    pub fn new() -> Self {
        AverageAnswer {
            key: 0,
            average: 0.0,
        }
    }
}

// structure to require Stat data from the Oracle
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct ReqStat {
    pub vector_id: u32,
}

// structure used to answer for stats request
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct StatAnswer {
    pub key: u32,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub deviation: f64,
    pub timestamp: u64,
}

impl StatAnswer {
    pub fn new() -> Self {
        StatAnswer {
            key: 0,
            average: 0.0,
            min: 0.0,
            max: 0.0,
            deviation: 0.0,
            timestamp: 0,
        }
    }
}

// structure to answer for statistic data (average,min,max,deviation)
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct OracleStatData {
    pub key: u32,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub deviation: f64,
    pub timestamp: u64,
}

impl OracleStatData {
    pub fn new() -> Self {
        OracleStatData {
            key: 0,
            average: 0.0,
            min: 0.0,
            max: 0.0,
            deviation: 0.0,
            timestamp: 0,
        }
    }
}


impl fmt::Display for OraoArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OraoArray: {} ", json!(self.data))
    }
}

impl fmt::Display for ReqArrayStruc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OraoArray: {} ", json!(self.vectors))
    }
}

pub fn OraoArray_new() -> OraoArrayTimestamp {
    let qt = OraoArrayTimestamp { provider: 0, data: vec! {}, timestamp: 0 };
    qt
}

pub fn OraoArray_newV3() -> OraoArrayTimestampV3 {
    let qt = OraoArrayTimestampV3 { provider: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], data: vec! {}, timestamp: 0 };
    qt
}


/// ProviderGaps
/// keeps track of gaps per provider.
pub struct ProviderGap {
    pub map: HashMap<ProviderId, i32>,
}

impl ProviderGap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn increment(&mut self, provider_id: ProviderId) {
        let count = self.map.entry(provider_id).or_insert(0);
        *count += 1;
    }

    pub fn average(&self) -> f32 {
        if self.map.len() == 0 {
            return 0.;
        }
        let sum = self.map.iter().map(|(_, &count)| count).sum::<i32>();
        sum as f32 / self.map.len() as f32
    }

    pub fn maximum(&self) -> i32 {
        self.map.iter().map(|(_, &count)| count).max().unwrap_or(0)
    }
}

pub type ProviderId = i32;
pub type Stream = Vec<Message>;

/// Input Stream represents data stream
/// coming from a data provider
#[derive(Clone, Debug)]
pub struct Message {
    pub key: i32,
    pub provider_id: i32,
    pub timestamp: i64,
    pub value: f32,
}

impl PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self.value.eq(&other.value)
    }
}

