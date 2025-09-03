# Blockchain Concept::

## Lyra Blockchain - How to start.
### Below we will go through a quick set up on beginning to use this blockchain, lets start with getting you a `wallet`.

# Creating a `wallet`
*step 1:*
Good thing about rust is that whether you have pre-built cargo or not, it wont matter because it will compile when required, so lets go straight and call the function we want to use, Type in your terminal the following command once you have directed yourself to the correct file.
```bash
cargo run wallet-create <name>
```
Currently we do NOT have secret phases set up to save with your new wallet once created - TODO:

# How to use this blockchain?
*step 1:*
Install and build.
```bash
cargo build
```

*step 2:*
Run the following command to use to test - Genesis block will be generated if a blockchain is not currently there which will be in a file called `./blockchain.json`.
```bash
cargo run block-new "block data here"
```
or
you can also use _ in between words if you don't want to use "String" 
```bash
cargo run block-new block_data_here
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
