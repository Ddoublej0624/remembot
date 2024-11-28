use server_core::logging::init_logging;
use server_runtime::runtime::init_runtime;

//Server spin-up procecure
fn main() {
    //TODO: Make these error messages not shitty.
    init_logging()
        .expect("Failed to init logging!");
    log::info!("Spinning up server..");
    init_runtime()
        .expect("Failed to spin up server runtime!");
}
