# Blockchain Concept::

## what are we doing different?
simply experimenting with different types and idea's on how blockchains can work in different ways.
let's break this down a little.

### Wallet
users are able to get a wallet, to keep things simple, we will be using an address which is in type of string, currency, amount, name so people are able to search up wallets via name instead of address, the address is what will determine whether it is the correct name or not.


# How to use this blockchain?
*step 1:*
Install and build.
```bash
cargo build
```

*step 2:*
Run the following command to use to test - Genesis block will be generated if a blockchain is not currently there which will be in a file called './blockchain.json'.
```bash
cargo run new-block "block data here"
```
or
you can also use _ in between words if you don't want to use "String" 
```bash
cargo run new-block block_data_here
```



## Testing your local network TCP port is active
in your terminal while you have the server going use the following command.
```bash 
nc -vz <server::port>
```
you should expect to see a result such as the following.
```bash
Connection to 127.0.0.1 port 8080 [tcp/http-alt] succeeded!
```
