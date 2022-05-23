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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postprocess() {
        let mut accounts = HashMap::new();
        accounts.insert(1, Account::new(1));
        accounts.insert(2, Account::new(2));

        let res = postprocess(accounts);
        assert!(res.is_ok());

        let raw_accounts = res.unwrap();
        assert_eq!(
            raw_accounts,
            vec![
                RawAccount::new(1, 0.0, 0.0, 0.0, false),
                RawAccount::new(2, 0.0, 0.0, 0.0, false)
            ]
        )
    }
}
