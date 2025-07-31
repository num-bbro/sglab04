use std::error::Error;

use askama::Template;
use askama_web::WebTemplate;
use axum::routing::get;

#[derive(Template, WebTemplate)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn hello() -> HelloTemplate {
    HelloTemplate {
        name: "world".to_string(),
    }
}

pub async fn run1() -> Result<(), Box<dyn Error>> {
    println!("run1");
    let app = axum::Router::new().route("/", get(hello));
    //let app = axum::Router::new().route("/:name/:age", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    /*
    axum::serve::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;
    */

    Ok(())
}
