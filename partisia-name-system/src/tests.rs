use crate::{
    actions::{
        execute_init, execute_mint, execute_record_delete, execute_record_delete_all,
        execute_record_mint, execute_record_update, execute_update_expiration,
    },
    msg::{
        PnsDomainUpdateExpirationMsg, PnsMintMsg, PnsRecordDeleteAllMsg, PnsRecordDeleteMsg,
        PnsRecordMintMsg, PnsRecordUpdateMsg,
    },
    state::{Record, RecordClass},
};

use utils::tests::{
    mock_contract_context, string_to_bytes, tomorrow_timestamp, yesterday_timestamp,
};

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = "name".to_string();
    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: domain.clone(),
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let num_token_id = state.get_token_id(&domain).unwrap();
    assert_eq!(num_token_id, 1);
}

#[test]
fn proper_mint_with_parent() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: "meta".to_string(),
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        token_id: 2,
        domain: "name".to_string(),
        parent_id: Some("meta".to_string()),
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let domains_length = state.domains.len();
    assert_eq!(domains_length, 2);
}

#[test]
#[should_panic(expected = "The specified domain is expired")]
fn when_parent_domain_is_expired_subdomain_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: "meta".to_string(),
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let update_expiration = PnsDomainUpdateExpirationMsg {
        domain: "meta".to_string(),
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_update_expiration(
        &mock_contract_context(minter),
        &mut state,
        &update_expiration,
    );

    let mint_msg = PnsMintMsg {
        token_id: 2,
        domain: "name".to_string(),
        parent_id: Some("meta".to_string()),
        expires_at: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "The specified domain is expired")]
fn when_parent_domain_is_expired_subdomain_record_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: "meta".to_string(),
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        token_id: 2,
        domain: "name".to_string(),
        parent_id: Some("meta".to_string()),
        expires_at: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let update_expiration = PnsDomainUpdateExpirationMsg {
        domain: "meta".to_string(),
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_update_expiration(
        &mock_contract_context(minter),
        &mut state,
        &update_expiration,
    );

    let record_mint_msg = PnsRecordMintMsg {
        domain: "name".to_string(),
        class: RecordClass::Wallet {},
        data: string_to_bytes("value"),
    };

    let _ = execute_record_mint(&mock_contract_context(minter), &mut state, &record_mint_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn when_parent_does_not_exist_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: "meta".to_string(),
        parent_id: Some("notfound".to_string()),
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Token with specified id is already minted")]
fn token_already_minted_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 2,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn mint_fails_when_parent_does_not_exist() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &"name".to_string();
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        parent_id: Some("not.existing.meta".to_string()),
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
fn proper_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &"name".to_string();
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = PnsRecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: string_to_bytes("data"),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class).unwrap();
    assert_eq!(
        *record,
        Record {
            data: string_to_bytes("data"),
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn when_token_not_present_record_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = PnsRecordMintMsg {
        domain: "not-existing.meta".to_string(),
        class: RecordClass::Wallet {},
        data: string_to_bytes("some data"),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
#[should_panic(expected = "Record with specified token id and class is already minted")]
fn record_already_minted_on_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = PnsRecordMintMsg {
        domain: "name".to_string(),
        class: RecordClass::Wallet {},
        data: string_to_bytes("some data"),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
fn proper_record_update() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &"name".to_string();
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = PnsRecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: string_to_bytes("data"),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_update_msg = PnsRecordUpdateMsg {
        domain: domain.clone(),
        class: record_class,
        data: string_to_bytes("new data"),
    };

    let _ = execute_record_update(
        &mock_contract_context(alice),
        &mut state,
        &record_update_msg,
    );

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class).unwrap();
    assert_eq!(
        *record,
        Record {
            data: string_to_bytes("new data"),
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn when_record_does_not_exist_record_update_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let record_update_msg = PnsRecordUpdateMsg {
        domain: "name".to_string(),
        class: RecordClass::Twitter {},
        data: string_to_bytes("new data"),
    };

    let _ = execute_record_update(
        &mock_contract_context(alice),
        &mut state,
        &record_update_msg,
    );
}

#[test]
#[should_panic(expected = "The specified domain is expired")]
fn when_domain_is_expired_record_mint_actions_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = PnsRecordMintMsg {
        domain: "name".to_string(),
        class: RecordClass::Wallet {},
        data: string_to_bytes("some data"),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
#[should_panic(expected = "The specified domain is expired")]
fn when_domain_is_expired_record_update_actions_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_update = PnsRecordUpdateMsg {
        domain: "name".to_string(),
        class: RecordClass::Wallet {},
        data: string_to_bytes("some data"),
    };

    let _ = execute_record_update(&mock_contract_context(alice), &mut state, &record_update);
}

#[test]
#[should_panic(expected = "The specified domain is expired")]
fn when_domain_is_expired_record_delete_actions_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_delete = PnsRecordDeleteMsg {
        domain: "name".to_string(),
        class: RecordClass::Wallet {},
    };

    let _ = execute_record_delete(&mock_contract_context(alice), &mut state, &record_delete);
}

#[test]
fn when_domain_is_expired_record_delete_all_action_works() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_delete_all = PnsRecordDeleteAllMsg {
        domain: "name".to_string(),
    };

    let _ = execute_record_delete_all(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_all,
    );

    let domain = state.get_domain("name").unwrap();
    assert!(domain.records.is_empty());
}

#[test]
fn when_domain_is_expired_update_expiration_action_works() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: "name".to_string(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(yesterday_timestamp()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let update_expiration = PnsDomainUpdateExpirationMsg {
        domain: "name".to_string(),
        expires_at: Some(tomorrow_timestamp()),
    };

    let _ = execute_update_expiration(
        &mock_contract_context(alice),
        &mut state,
        &update_expiration,
    );

    let domain = state.get_domain("name").unwrap();
    assert!(domain.expires_at == Some(tomorrow_timestamp()));
}

#[test]
fn proper_record_delete() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &"name".to_string();
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        parent_id: None,
        expires_at: Some(tomorrow_timestamp()),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = PnsRecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: string_to_bytes("data"),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_delete_msg = PnsRecordDeleteMsg {
        domain: domain.clone(),
        class: record_class,
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class);

    assert!(record.is_none());
}

#[test]
#[should_panic(expected = "Not found")]
fn when_record_does_not_exist_record_delete_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let record_delete_msg = PnsRecordDeleteMsg {
        domain: "name".to_string(),
        class: RecordClass::Twitter {},
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );
}
