//! hypervisor-engine — Sorabbyngo OS VM isolation layer

mod vm;
mod memory;
mod cpu;
mod devices;
mod snapshot;
mod kvm;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("hypervisor-engine starting");
    Ok(())
}
