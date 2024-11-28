use crate::server::{server_main, DEFAULT_BIND_ADDR};

//TODO:Everything! This file will contain all of the code responsible for setting up the
//architecture to handle incoming connections, for now..literally just call server main :)
//In the future, we probably want runner to call server_main, maybe we want to make server_main a
//guarded main, to do that we would initialize some SEH hooks here or something, and then we'd
//still call server_main here
pub fn init_runtime() -> anyhow::Result<()> {
    server_main(DEFAULT_BIND_ADDR)
}
