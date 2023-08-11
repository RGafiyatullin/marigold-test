use crate::data;
use crate::data::{Account, AccountID, EntryID, Space, SpaceID};
use crate::kernel::Kernel;
use crate::message::{AccountRegister, SpaceCreate, TodoEntrySet, TodoMessage};
use crate::signing::{Signed, VerificationError};

#[derive(Debug, thiserror::Error)]
pub enum MessageDispatchFailure {
    #[error("Not Implemented")]
    NotImplemented,

    #[error("Not Allowed")]
    NotAllowed,

    #[error("No Account")]
    NoAccount,

    #[error("Verification Error")]
    VerificationError(#[source] VerificationError),
}

pub fn process_inbound_message(
    kernel: impl Kernel,
    message: TodoMessage,
) -> Result<(), MessageDispatchFailure> {
    match message {
        TodoMessage::AccountRegister(account_register) =>
            process_account_register(kernel, account_register),

        TodoMessage::SpaceCreate(by_account, space_id, space_create) =>
            process_space_create(kernel, by_account, space_id, space_create),

        TodoMessage::TodoEntrySet(by_account, space_id, entry_id, entry_set) =>
            process_todo_entry_set(kernel, by_account, space_id, entry_id, entry_set),
    }
}

fn process_account_register(
    kernel: impl Kernel,
    account_register: AccountRegister,
) -> Result<(), MessageDispatchFailure> {
    let public_key = &account_register.public_key;
    let () = account_register
        .signed
        .verify(public_key)
        .map_err(MessageDispatchFailure::VerificationError)?;

    let account_entry = Account { public_key: public_key.to_owned() };

    let path = data::path_account(&account_register.account_id);

    // TODO: handle gracefully
    data::store_new(kernel, path, account_entry).expect("ew..");

    Ok(())
}

fn process_space_create(
    mut kernel: impl Kernel,
    by_account: AccountID,
    space_id: SpaceID,
    space_create: Signed<SpaceCreate>,
) -> Result<(), MessageDispatchFailure> {
    let account_path = data::path_account(&by_account);
    let space_path = data::path_space(&space_id);

    let Some(account_entry) = data::read::<Account>(&mut kernel, account_path).expect("ew...")
    else {
        return Err(MessageDispatchFailure::NoAccount)
    };

    let space_create = space_create
        .verify(&account_entry.public_key)
        .map_err(MessageDispatchFailure::VerificationError)?;

    if !space_create.accounts.iter().any(|a| a == &by_account) {
        return Err(MessageDispatchFailure::NotAllowed)
    }

    let space_entry = Space { accounts: space_create.accounts.to_owned() };
    data::store_new(kernel, space_path, space_entry).expect("ew...");

    Ok(())
}

fn process_todo_entry_set(
    _kernel: impl Kernel,
    _by_account: AccountID,
    _space_id: SpaceID,
    _entry_id: EntryID,
    _entry_set: Signed<TodoEntrySet>,
) -> Result<(), MessageDispatchFailure> {
    Err(MessageDispatchFailure::NotImplemented)
}
