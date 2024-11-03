use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Decimal, Int4, Text, Timestamp, Varchar};
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub tx_hash: String,
    pub block_id: i32,
    pub sender_address: String,
    pub receiver_address: String,
    pub value: BigDecimal, // Requires bigdecimal crate
    pub gas_price: i64,
    pub gas_used: i64,
    pub nonce: i64,
    pub input_data: Option<String>,
    pub created_at: NaiveDateTime,
}

// Diesel schema (use diesel CLI to generate schema.rs)
table! {
    transactions (id) {
        id -> Int4,
        tx_hash -> Varchar,
        block_id -> Int4,
        sender_address -> Varchar,
        receiver_address -> Varchar,
        value -> Numeric,
        gas_price -> BigInt,
        gas_used -> BigInt,
        nonce -> BigInt,
        input_data -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
