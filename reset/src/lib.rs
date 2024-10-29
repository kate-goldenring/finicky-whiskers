use anyhow::{Context, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    key_value::Store,
    sqlite::Connection,
};

#[http_component]
fn reset(_req: Request) -> Result<Response> {
    if let Err(e) = reset_keyvalue() {
        return Ok(Response::builder().status(500).body(e.to_string()).build());
    }
    if let Err(e) = reset_highscore() {
        return Ok(Response::builder().status(500).body(e.to_string()).build());
    }
    Ok(Response::builder()
        .status(200)
        .body("Finicky Whickers is reset.")
        .build())
}

fn reset_keyvalue() -> Result<()> {
    let store = Store::open_default().with_context(|| "Failed to open default key-value store")?;
    let keys = store
        .get_keys()
        .with_context(|| "Failed to get keys from key-value store")?;

    keys.into_iter()
        .filter(|key| key.starts_with("fw-"))
        .try_for_each(|key| {
            store
                .delete(&key)
                .with_context(|| "Failed to delete {key} from key-value store")
        })?;

    Ok(())
}

fn reset_highscore() -> Result<()> {
    let conn = Connection::open_default()?;
    let query = "DELETE FROM highscore";
    conn.execute(query, &[])?;
    Ok(())
}
