use std::collections::HashMap;

use axum::{routing::{get, post}, Router, response::IntoResponse};
use time::OffsetDateTime;

#[derive(Default)]
struct Account {
    balance: i64,
    limit: i64,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn with_limit(limit: i64) -> Self {
        Account { limit, ..Default::default() }
    }
}

enum TransactionType {
    Credit,
    Debit,
}
struct Transaction {
    value: i64,
    kind: TransactionType,
    description: String,
    date: OffsetDateTime,
}

#[tokio::main]
async fn main() {
    let account = HashMap::<u8, Account>::from_iter([
        (1, Account::with_limit(100_00)),
        (2, Account::with_limit(80_00)),
        (3, Account::with_limit(1_000_000)),
        (4, Account::with_limit(10_000_000)),
        (5, Account::with_limit(500_000)),
    ]);

    let app = Router::new()
        .route("/clientes/:id/transacoes", post(create_transaction))
        .route("/clientes/:id/extrato", get(view_account));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_transaction() -> impl IntoResponse {
    "Transaction created"
}

async fn view_account() -> impl IntoResponse {
    "This is your account"
}