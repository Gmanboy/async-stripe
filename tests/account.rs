mod mock;

#[test]
fn is_account_listable() {
    mock::with_client(|client| {
        let result = stripe::Account::list(client, &ListAccounts::new());
        let list = match result {
            Err(err) => panic!("{}", err),
            Ok(ok) => ok,
        };

        // Check to ensure we are deserializing _something_ and this test
        // actually validates something worthwhile.
        assert!(list.len() > 0);
    });
}
