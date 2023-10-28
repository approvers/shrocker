use std::time::Duration;

use anyhow::Result;
use indoc::indoc;

use crate::report::Reporter;

pub async fn perform_register(user: &str, public_key: &str, reporter: &mut impl Reporter) -> Result<()> {
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(())
}