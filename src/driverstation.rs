use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use tokio::task::AbortHandle;
use tokio::time::{self, Duration, Interval};

#[derive(Clone, Copy)]
pub struct TeamNumber(u16);

impl TeamNumber {
    pub fn new(number: u16) -> Self {
        assert!(number > 0 && number <= 9999);

        Self(number)
    }
}

pub struct DriverStation {
    team_number: TeamNumber,
    socket: Option<UdpSocket>,
    quit: bool,
    count: u16,
    fms_connected: bool,
    connection: Option<AbortHandle>,
}

impl Default for DriverStation {
    fn default() -> Self {
        Self {
            team_number: 0.into(),
            socket: None,
            quit: false,
            count: 0,
            fms_connected: false,
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

    fn ds_to_fms() {}
}

impl From<TeamNumber> for Ipv4Addr {
    fn from(value: TeamNumber) -> Self {
        let last_digits = ((value.0 as f64) / 100.0).floor();

        Ipv4Addr::new(
            10,
            last_digits as u8,
            ((value.0 as f64) - last_digits * 100.0) as u8,
            2,
        )
    }
}

impl From<u16> for TeamNumber {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}
