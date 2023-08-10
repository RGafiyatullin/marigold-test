use tezos_data_encoding::types::Bytes;
use tezos_smart_rollup_mock::MockHost;

#[test]
fn t01() {
    let mut host = MockHost::default();
    todo_kernel::entry(&mut host);
}

#[test]
fn t02() {
    let mut host = MockHost::default();
    host.add_external(Bytes::from(vec![42, 42, 42]));
    todo_kernel::entry(&mut host);
}

#[test]
fn t03() {
    let mut host = MockHost::default();
    host.add_external(Bytes::from(vec![todo_kernel::message::MAGIC]));
    todo_kernel::entry(&mut host);
}

#[test]
fn t04() {
    let mut message = vec![todo_kernel::message::MAGIC];
    message.extend(
        r#"
    {"create": {
        "public_key": "asdf",
        "signature": "asdf",
        "nonce": 1,
        "data": {}
    }}
    "#
        .as_bytes(),
    );

    let mut host = MockHost::default();
    host.add_external(Bytes::from(message));
    todo_kernel::entry(&mut host);
}
