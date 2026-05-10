use anyhow::Result;
use mpris_server::Server;

use crate::app::SharedState;

pub mod player_interface;
pub mod root_interface;

const BUS_NAME: &str = "AmiMusic";

pub struct Mpris {
    pub shared_state: SharedState,
}

impl Mpris {
    pub fn new(shared_state: SharedState) -> Mpris {
        Mpris { shared_state }
    }

    pub async fn start(self) -> Result<Server<Mpris>> {
        Ok(Server::new(&format!("org.mpris.MediaPlayer2.{}", BUS_NAME), self).await?)
    }
}
