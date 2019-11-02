
# Monzo

[![Build Status](https://travis-ci.org/danieleades/monzo-lib.svg?branch=master)](https://travis-ci.org/danieleades/monzo-lib)
[![Latest Docs](https://docs.rs/monzo-lib/badge.svg)](https://docs.rs/monzo-lib/)

This crate is a Monzo client in pure rust.

It's ergonomic, strongly-typed, and asynchronous.

The majority of the endpoints are already supported. If you need a piece of
functionality that is not yet implemented, please open an issue or even
better, a pull request.

In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the docs](https://docs.monzo.com/))

### Usage
```rust
use monzo::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {

    // You can create a simple monzo client using only an access token
    let quick_client = Client::quick("ACCESS_TOKEN");

    // get a list of accounts
    let accounts = quick_client.accounts().send().await?;

    // get the id of one of the accounts
    let account_id = &accounts[0].id;

    // get the balance of that account
    let balance = quick_client.balance(account_id).send().await?;

    // If you have a refresh token and client credentials
    // you can create or upgrade a client which is capable
    // of refreshing its own access token.
    let mut refreshable_client = quick_client.with_refresh_tokens(
        "CLIENT_ID",
        "CLIENT_SECRET",
        "REFRESH_TOKEN",
    );

    refreshable_client.refresh_auth().await?;

    Ok(())
}
```

---

License: Apache-2.0
