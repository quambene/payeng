use crate::models::{Account, RawAccount};
use std::collections::HashMap;

pub fn postprocess(accounts: HashMap<u16, Account>) -> Result<Vec<RawAccount>, anyhow::Error> {
    let mut raw_accounts: Vec<RawAccount> = vec![];

    for (_client_id, account) in accounts {
        let raw_account = account.into();
        raw_accounts.push(raw_account);
    }

    Ok(raw_accounts)
}
