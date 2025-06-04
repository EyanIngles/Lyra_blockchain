// This file will be used for dealing with the args being parsed in.

// enum list to give us a selection of what to do such as get_block enum value that if the args being parsed in make this the path to take,
// we will set that enum to true.

#[derive(PartialEq, Debug)]
pub enum Path {
    StartServer,
    CreateWallet,
    GetWallet,
    GetBlock,
    NewBlock,
    WalletLogin,
    WalletLogout,
    ImportWallet,
    Command,
    // help: bool, // should have a help command to show how to use or show what commands are available.
}
// pub enum Args_process { // this is to catch any errors if needed and used to also maybe as the process is waiting?
//     Success,
//     Fail,
//     Proceeding,
//     Error,
// }

pub fn sort_client_args_direction(input: &str) -> Path {
    match input {
        "server" => Path::StartServer,
        "block-get" => Path::GetBlock,
        "block-new" => Path::NewBlock,
        "wallet-create" => Path::CreateWallet,
        "wallet-get" => Path::GetWallet,
        "wallet-login" => Path::WalletLogin,
        "wallet-logout" => Path::WalletLogout,
        "wallet-import" => Path::ImportWallet,
        "command" => Path::Command,
        &_ => todo!("Err: Unsupported argument passed"),
    }
}

#[test] // TODO: update test to ensure all paths are on here.
pub fn getting_args_correctly() {
    let (server_arg, get_block_arg, create_wallet_arg, get_wallet_arg, get_new_block) = (
        "server",
        "block-get",
        "wallet-create",
        "wallet-get",
        "block-new",
    );
    // testing to see if the server arg works.
    let server = Path::StartServer;
    let server_enum = sort_client_args_direction(server_arg);
    assert_eq!(server_enum, server);

    let block = Path::GetBlock;
    let block_enum = sort_client_args_direction(get_block_arg);
    assert_eq!(block_enum, block);
    assert_ne!(block_enum, server);

    let wallet = Path::CreateWallet;
    let wallet_enum = sort_client_args_direction(create_wallet_arg);
    assert_ne!(wallet_enum, block);
    assert_ne!(wallet_enum, server);
    assert_eq!(wallet_enum, wallet);

    let getting_wallet = Path::GetWallet;
    let get_wallet_enum = sort_client_args_direction(get_wallet_arg);
    assert_ne!(get_wallet_enum, block);
    assert_ne!(get_wallet_enum, server);
    assert_ne!(get_wallet_enum, wallet);
    assert_eq!(get_wallet_enum, getting_wallet);

    let new_block = Path::NewBlock;
    let new_block_enum = sort_client_args_direction(get_new_block);
    assert_ne!(new_block_enum, block);
    assert_ne!(new_block_enum, server);
    assert_ne!(new_block_enum, wallet);
    assert_ne!(new_block_enum, getting_wallet);
    assert_eq!(new_block_enum, new_block);
}

#[test]
#[should_panic]
pub fn panic_when_no_command_is_used() {
    let _no_enum = sort_client_args_direction("no_command");
}
