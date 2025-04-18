#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/").await?.print().await?;

    hc.do_post("/signup",
        (r#"{
            "name": "piet4",
            "email": "fred@zwietz.nl",
            "password": "P@ssW0rd"
            }
            "#,
            "application/json"
        ),)
    .await?.print().await?;

    Ok(())
}