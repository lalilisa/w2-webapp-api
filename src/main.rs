

use tracing::{error};
use w4::{start_server};
#[tokio::main]
async  fn main()-> anyhow::Result<()>{
    dotenv::dotenv().ok();

    if let Err(e) = start_server().await {
        error!("Failed to start application: {}", e);
    };

    Ok(())
}
