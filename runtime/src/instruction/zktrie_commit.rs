use crate::RuntimeContext;
use fluentbase_rwasm::{common::Trap, Caller};

pub struct ZkTrieCommit;

impl ZkTrieCommit {
    pub fn fn_handler<T>(mut caller: Caller<'_, RuntimeContext<T>>) -> Result<(), Trap> {
        Ok(())
    }

    pub fn fn_impl() {}
}
