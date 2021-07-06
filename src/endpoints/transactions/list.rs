use super::{Pagination, Since, Transaction};
use crate::{
    client::{self, send_and_resolve_request},
    endpoints::Endpoint,
    Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A request to retrieve a list of transactions from the Monzo API
///
/// Use the builder-style methods to set optional fields on the request
#[derive(Debug)]
pub struct Request<'a> {
    client: &'a dyn client::Inner,
    form: Form<'a>,
}

impl<'a> Endpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> &str {
        "https://api.monzo.com/transactions"
    }

    fn form(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.form)
    }
}

impl<'a> Request<'a> {
    pub(crate) fn new(client: &'a dyn client::Inner, account_id: &'a str) -> Self {
        let form = Form {
            account_id,
            pagination: Pagination::default(),
            expand_merchant: None,
        };

        Self { client, form }
    }

    /// Only return transactions which occurred after the given `DateTime`
    pub fn since(mut self, datetime: DateTime<Utc>) -> Self {
        self.form.pagination.since = Some(Since::Timestamp(datetime));
        self
    }

    /// Only return transactions which occurred after the given transaction.
    ///
    /// This can be used for paginating.
    pub fn since_transaction(mut self, transaction_id: String) -> Self {
        self.form.pagination.since = Some(Since::ObjectId(transaction_id));
        self
    }

    /// Only return transactions which occurred before a given `DateTime`
    pub fn before(mut self, datetime: DateTime<Utc>) -> Self {
        self.form.pagination.before = Some(datetime);
        self
    }

    /// Set the maximum number of transactions to be returned
    pub fn limit(mut self, limit: u16) -> Self {
        self.form.pagination.limit = Some(limit);
        self
    }

    /// Optionally expand the merchant field from an id string into a struct
    /// container merchant details
    pub fn expand_merchant(mut self) -> Self {
        self.form.expand_merchant = Some("merchant");
        self
    }

    pub async fn send(self) -> Result<Vec<Transaction>> {
        #[derive(Deserialize)]
        struct Response {
            transactions: Vec<Transaction>,
        }

        let response: Response = send_and_resolve_request(self.client, &self).await?;

        Ok(response.transactions)
    }
}

#[derive(Serialize, Debug)]
struct Form<'a> {
    account_id: &'a str,

    #[serde(flatten)]
    pagination: Pagination,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expand[]")]
    expand_merchant: Option<&'a str>,
}

#[derive(Deserialize)]
pub(crate) struct Response {
    transactions: Vec<Transaction>,
}

impl From<Response> for Vec<Transaction> {
    fn from(response: Response) -> Self {
        response.transactions
    }
}
