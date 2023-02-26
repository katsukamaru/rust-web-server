use axum::{response::Html, routing::{get, post}, Router, Form};
use std::net::SocketAddr;
use serde::Deserialize;

#[tokio::main]  // main関数を非同期関数にするために必要
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rustwi=debug")
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(login))
        .route("/sign_up", post(accept_form));
    ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login() -> Html<&'static str> {
    let a = r#"
    <form action="/sign_up" method="post">
        <input type="text" name="username" placeholder="ユーザー名" />
        <input type="password" name="password" placeholder="パスワード" />
        <button type="submit">送信</button>
    </form>"#;
    Html(a)
}

#[derive(Deserialize)]  // Form<T>のTはserde::Deserializeを実装している必要がある
struct SignUp {
    username: String,
    password: String,
}

async fn accept_form(form: Form<SignUp>) {
    let sign_up: SignUp = form.0;
    // DBに保存するなど
}