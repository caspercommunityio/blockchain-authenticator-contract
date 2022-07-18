
<p align="center"><a href="https://analytics.caspercommunity.io" target="_blank"><img src="https://analytics.caspercommunity.io/assets/icon/android-chrome-512x512.png" width="150"></a></p>

## About Blockchain Authenticator Contract

Provide a simple smart contract written in Rust to store a list of String.<br>
This contract is used by the [Blockchain Authenticator App](https://github.com/caspercommunityio/blockchain-authenticator-app) to store the secret keys needed to generate the OTP password.

The contract must be called with 3 parameters :
- named-key : Name of the property where the data is stored
- method :
  - add : Add the "keys" to the current list of string
  - del : Remove the "keys" from the current list of string
  - dellall : Remove all elements from the list of string
- keys : List of strings

## How to install

First, be sure that you have the needed tools installed.

You can check the Casper's [documentation here](https://docs.casperlabs.io/dapp-dev-guide/writing-contracts/getting-started/)

Here are the version of the tools that we've used to build and run the smartcontract :

```
rustup --version
rustup 1.24.3 (ce5817a94 2021-05-31)

cmake --version
cmake version 3.22.1

```

You also need [Git](https://git-scm.com/downloads) to clone the repository.

To build the smartcontract :

```
git clone git@github.com:caspercommunityio/blockchain-authenticator-contract.git
cd blockchain-authenticator-contract
make prepare
make build-contract
```

## How to run the tests

Follow the "how to install" part and then :

```
cd blockchain-authenticator-contract
make test
```
The output should be something like this :

```
make test
cd contract && cargo build --release --target wasm32-unknown-unknown
    Finished release [optimized] target(s) in 0.03s
wasm-strip contract/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
mkdir -p tests/wasm
cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
cd tests && cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests (target/debug/deps/integration_tests-5924344271c8fdee)

running 8 tests
test tests::should_create_named_keys ... ok
test tests::should_panic_missing_parameters - should panic ... ok
test tests::should_add_multiple_elements ... ok
test tests::should_add_non_existing_element ... ok
test tests::should_not_del_element_non_existing_element ... ok
test tests::should_del_element ... ok
test tests::should_del_multiple_elements ... ok
test tests::should_update_existing_element ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
```

## How to test on testnet

Install the "casper-client" using cargo :

```
cargo +nightly-2022-02-03 install casper-client --locked
```

Check that the "casper-client" is correctly installed :

```
casper-client --version
Casper client 1.5.0
```

Create a new account with [Casper Signer](https://docs.cspr.community/docs/user-guides/SignerGuide.html) and claim tokens using the faucet on https://testnet.cspr.live.

Then download your .pem file using the casper signer.

To deploy it on testnet, we have to use the following command :

```
casper-client put-deploy --chain-name casper-test -n http://95.216.67.162:7777 -k /path/to/your/secret/keys/private.pem -p 1000000000 -s /path/to/your/project/contract/target/wasm32-unknown-unknown/release/contract.wasm --session-args-complex /path/to/your/args.txt
```

As you can see, we use a file to store the args because the type CLList is considered complex.

The content of the file should be like this :

```
[
 {
	"name" : "method",
	"value" : {
		"raw_bytes" : "07000000030000006164640a"
	}
 },{
	"name" : "keys",
	"value" : {
		"raw_bytes" : "1100000001000000090000004944313b56414c55450e0a"
	}
 },{
	"name" : "named-key",
	"value" : {
		"raw_bytes" : "0800000004000000746573740a"
	}
 }
]
```

The values are encoded like specified in the Casper's documentation : https://docs.casperlabs.io/design/serialization-standard/#clvalue-clvalue

Here are some raw_bytes values for the parameters :

```
String "test" for the named-key parameter => 0800000004000000746573740a
String "add" => 07000000030000006164640a
String "del" => 070000000300000064656c0a
List of 1 Element "ID1;VALUE" => 1100000001000000090000004944313b56414c55450e0a
List of 2 Elements "ID1;VALUE, ID2;VALUE" => 1e00000002000000090000004944313b56414c5545090000004944323b56414c55450e0a
List of 3 Elements "ID1;VALUE, ID2;VALUE, ID3;VALUE" => 2b00000003000000090000004944313b56414c5545090000004944323b56414c5545090000004944333b56414c55450e0a
```

With these examples, you can try different tests scenarios on testnet.

The result of the deploy should be like this :

```
{
  "id": -6078809224594666102,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.4.6",
    "deploy_hash": "c60402ece049fb41f60ae67bedeec4c0bd71b7fb0cec0d3267fd030eededa71e"
  }
}
```

You can check the result of the deploy on https://testnet.cspr.live.

## How to write the contract for another blockchain

The smart contract must store an array of string with the following format "ID;VALUE".
Then when you call the smartcontract, you must have at least 2 parameters :
- method :
  - add : Add the "keys" to the current list of string
  - del : Remove the "keys" from the current list of string
  - dellall : Remove all elements from the list of string
- keys : List of strings
- named-key (optional) : Name of the property where the data is stored

Then [the webapp](https://github.com/caspercommunityio/blockchain-authenticator-app) must be adapted to call and retrieve the data from that blockchain.

## License

The Blockchain Authenticator Contract package is an open-sourced software licensed under the [MIT license](https://opensource.org/licenses/MIT).
