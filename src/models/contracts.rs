use serde_json::Value;
use diesel::{Queryable, Insertable};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "contracts"]
pub struct Contract {
    pub id: i32,
    pub contract_address: String,
    pub creator_address: String,
    pub source_code: Option<String>,
    pub abi: Option<Value>,      // JSONB field
    pub bytecode: Option<String>,
    pub created_at: NaiveDateTime,
}

table! {
    contracts (id) {
        id -> Int4,
        contract_address -> Varchar,
        creator_address -> Varchar,
        source_code -> Nullable<Text>,
        abi -> Nullable<Jsonb>,
        bytecode -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
