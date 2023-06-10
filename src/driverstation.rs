use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use tokio::task::AbortHandle;
use tokio::time::{self, Duration, Interval};

use crate::teamnumber::TeamNumber;

pub struct DriverStation {
    team_number: TeamNumber,
    socket: Option<UdpSocket>,
    quit: bool,
    count: u16,
    connection: Option<AbortHandle>,
}

impl Default for DriverStation {
    fn default() -> Self {
        Self {
            team_number: 0.into(),
            socket: None,
            quit: false,
            count: 0,
            connection: None,
        }
    }
}

impl DriverStation {
    pub fn new(team: u16) -> Self {
        let mut ds = Self::default();

        ds.team_number = team.into();

        ds
    }

    pub async fn init(&mut self) {
        self.socket = UdpSocket::bind::<(Ipv4Addr, u16)>((self.team_number.into(), 3000u16))
            .await
            .ok();

        self.connection = Some(
            tokio::spawn(async {
                let mut update_rio: Interval = time::interval(Duration::from_millis(20));

                loop {
                    DriverStation::ds_to_rio().await;
                    update_rio.tick().await;
                }
            })
            .abort_handle(),
        );
    }

    pub fn quit(self) {
        if let Some(connection) = self.connection {
            // if the connection is none we can assume is was never made
            connection.abort();
        }
    }

    async fn ds_to_rio() {}
}
