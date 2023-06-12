use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {

#[macro_use]
extern crate dotenv_codegen;

    pub mod server;

    #[tokio::main]
    async fn main() {
        server::server_main().await
    }
}

}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
