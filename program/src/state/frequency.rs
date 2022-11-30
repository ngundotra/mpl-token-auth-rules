use borsh::{BorshDeserialize, BorshSerialize};

use super::{Key, SolanaAccount};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct FrequencyAccount {
    pub key: Key,
    pub last_update: i64,
    pub period: i64,
}

impl SolanaAccount for FrequencyAccount {
    fn key() -> Key {
        Key::Frequency
    }

    fn size() -> usize {
        0
    }
}