use spacetimedb_sdk::{DbContext, Error, Identity, credentials};

mod bindings;
use bindings::*;

/// The URI of the SpacetimeDB instance hosting our chat database and module.
const HOST: &str = "wss://bitcraft-early-access.spacetimedb.com";

/// The database name we chose when we published our module.
const DB_NAME: &str = "bitcraft-5";

fn main() {
    // Connect to the database
    let ctx = connect_to_db();

    // Subscribe to SQL queries in order to construct a local partial replica of the database.
    subscribe_to_tables(&ctx);

    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();
}

/// Load credentials from a file and connect to the database.
fn connect_to_db() -> DbConnection {
    DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        .with_token(Some("eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJoZXhfaWRlbnRpdHkiOiJjMjAwNjU5YWQxMDA1ODdhNGViOGZiMGZlYTRiOGVlYzUzZDIxYWY4NDM5OTk2Yjk3ZDFjOGM4MmVkMWI5ZTQzIiwic3ViIjoiZDJmNjRlZjYtMjk2YS00ZGFiLWIzNzMtZjM3NjJjZTUxZDczIiwiaXNzIjoibG9jYWxob3N0IiwiYXVkIjpbInNwYWNldGltZWRiIl0sImlhdCI6MTc1MDUxMTAwOCwiZXhwIjpudWxsfQ.Ph2SBG89BHgttYcL-qItaf-o_M5W87sEarz9lKq6AeS-2YzVZmWjyZj4dIFtMWcQq07WT8UCvA0P90TXXXersg"))
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

fn creds_store() -> credentials::File {
    credentials::File::new("bittravel")
}

/// Our `on_connect` callback: save our credentials to a file.
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }

    println!("Connected!");
}

/// Our `on_connect_error` callback: print the error, then exit the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}

/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_applied(on_sub_applied)
        .on_error(on_sub_error);
    //.subscribe(["SELECT * FROM user", "SELECT * FROM message"]);
}

/// Our `on_subscription_applied` callback:
/// sort all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &SubscriptionEventContext) {
    println!("Fully connected and all subscriptions applied.");
    println!("Use /name to set your name, or type a message!");
}

/// Or `on_error` callback:
/// print the error, then exit the process.
fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
    std::process::exit(1);
}

// wss://bitcraft-early-access.adokkf74uopr5hao3ww3hejuu.com
// wss://bitcraft-early-access.spacetimedb.com
// eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJoZXhfaWRlbnRpdHkiOiJjMjAwNjU5YWQxMDA1ODdhNGViOGZiMGZlYTRiOGVlYzUzZDIxYWY4NDM5OTk2Yjk3ZDFjOGM4MmVkMWI5ZTQzIiwic3ViIjoiZDJmNjRlZjYtMjk2YS00ZGFiLWIzNzMtZjM3NjJjZTUxZDczIiwiaXNzIjoibG9jYWxob3N0IiwiYXVkIjpbInNwYWNldGltZWRiIl0sImlhdCI6MTc1MDUxMTAwOCwiZXhwIjpudWxsfQ.Ph2SBG89BHgttYcL-qItaf-o_M5W87sEarz9lKq6AeS-2YzVZmWjyZj4dIFtMWcQq07WT8UCvA0P90TXXXersg
