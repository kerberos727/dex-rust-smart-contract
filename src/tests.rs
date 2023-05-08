use partisia_name_system::{
    msg::{
        PnsApproveForAllMsg, PnsApproveMsg, PnsBurnMsg, PnsCheckOwnerMsg, PnsMintMsg, PnsMultiMintMsg,
        RecordDeleteMsg, RecordMintMsg, RecordUpdateMsg, PnsRevokeForAllMsg, PnsRevokeMsg, PnsSetBaseUriMsg,
        PnsTransferFromMsg, PnsTransferMsg, PnsUpdateMinterMsg,
    },
    state::RecordClass,
};

use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};

use utils::events::IntoShortnameRPCEvent;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

const TRANSFER: u32 = 0x01;
const TRANSFER_FROM: u32 = 0x03;
const APPROVE: u32 = 0x05;
const SET_BASE_URI: u32 = 0x07;
const MINT: u32 = 0x09;
const APPROVE_FOR_ALL: u32 = 0x11;
const REVOKE: u32 = 0x13;
const REVOKE_FOR_ALL: u32 = 0x15;
const BURN: u32 = 0x17;

const CHECKOWNER: u32 = 0x18;
const UPDATE_MINTER: u32 = 0x19;
const MULTI_MINT: u32 = 0x20;
const RECORD_MINT: u32 = 0x21;
const RECORD_UPDATE: u32 = 0x22;
const RECORD_DELETE: u32 = 0x23;
#[test]
fn proper_transfer_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsTransferMsg {
        to: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };
    let mut event_group = EventGroup::builder();
    let mut test_event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsTransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsApproveMsg {
        spender: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_set_base_uri_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsSetBaseUriMsg {
        new_base_uri: "new".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(SET_BASE_URI))
        .argument("new".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsMintMsg {
        token_id: "name.meta".to_string(),
        to: mock_address(1u8),
        token_uri: None,
        parent_id: Some("".to_string()),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MINT))
        .argument("name.meta".to_string())
        .argument(mock_address(1u8))
        .argument(None::<String>)
        .argument(Some("".to_string()))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = RecordMintMsg {
        token_id: "name.meta".to_string(),
        class: RecordClass::Wallet {},
        data: "".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(RECORD_MINT))
        .argument("name.meta".to_string())
        .argument(RecordClass::Wallet {})
        .argument("".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_update_action_call() {
    let dest = mock_address(30u8);

    let msg = RecordUpdateMsg {
        token_id: "name.meta".to_string(),
        class: RecordClass::Wallet {},
        data: "".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(RECORD_UPDATE))
        .argument("name.meta".to_string())
        .argument(RecordClass::Wallet {})
        .argument("".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_delete_action_call() {
    let dest = mock_address(30u8);

    let msg = RecordDeleteMsg {
        token_id: "name.meta".to_string(),
        class: RecordClass::Wallet {},
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(RECORD_DELETE))
        .argument("name.meta".to_string())
        .argument(RecordClass::Wallet {})
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_ownership_check_call() {
    let dest = mock_address(30u8);

    let msg = PnsCheckOwnerMsg {
        owner: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(CHECKOWNER))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsApproveForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRevokeMsg {
        spender: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(REVOKE))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRevokeForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(REVOKE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsBurnMsg {
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BURN))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_minter_update_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsUpdateMinterMsg {
        new_minter: mock_address(19u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(UPDATE_MINTER))
        .argument(mock_address(19u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_multi_mint_action_call() {
    let dest = mock_address(30u8);

    let mints = vec![
        PnsMintMsg {
            token_id: "name.meta".to_string(),
            to: mock_address(4),
            parent_id: Some("".to_string()),
            token_uri: None,
        },
        PnsMintMsg {
            token_id: "name2.meta".to_string(),
            to: mock_address(4),
            parent_id: Some("".to_string()),
            token_uri: None,
        },
        PnsMintMsg {
            token_id: "name3.meta".to_string(),
            to: mock_address(5),
            parent_id: Some("".to_string()),
            token_uri: None,
        },
        PnsMintMsg {
            token_id: "name4.meta".to_string(),
            to: mock_address(5),
            parent_id: Some("".to_string()),
            token_uri: None,
        },
        PnsMintMsg {
            token_id: "name5.meta".to_string(),
            to: mock_address(6),
            parent_id: Some("".to_string()),
            token_uri: None,
        },
    ];
    let msg = PnsMultiMintMsg {
        mints: mints.clone(),
    };
    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MULTI_MINT))
        .argument(mints)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
