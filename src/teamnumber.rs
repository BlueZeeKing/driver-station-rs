use std::net::Ipv4Addr;

#[derive(Clone, Copy)]
pub struct TeamNumber(u16);

impl TeamNumber {
    pub fn new(number: u16) -> Self {
        assert!(number > 0 && number <= 9999);

        Self(number)
    }
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

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};

    use super::TeamNumber;

    #[test]
    fn one_digit_team_number() {
        let team_number = TeamNumber::new(8);

        assert_eq!(
            Ipv4Addr::from(team_number),
            "10.0.8.2".parse::<Ipv4Addr>().unwrap()
        )
    }

    #[test]
    fn two_digit_team_number() {
        let team_number = TeamNumber::new(82);

        assert_eq!(
            Ipv4Addr::from(team_number),
            Ipv4Addr::from_str("10.0.82.2").unwrap()
        )
    }

    #[test]
    fn three_digit_team_number() {
        let team_number = TeamNumber::new(823);

        assert_eq!(
            Ipv4Addr::from(team_number),
            Ipv4Addr::from_str("10.8.23.2").unwrap()
        )
    }

    #[test]
    fn four_digit_team_number() {
        let team_number = TeamNumber::new(8230);

        assert_eq!(
            Ipv4Addr::from(team_number),
            Ipv4Addr::from_str("10.82.30.2").unwrap()
        );

        assert_eq!(
            Ipv4Addr::from(TeamNumber::new(9033)),
            Ipv4Addr::from_str("10.90.33.2").unwrap()
        )
    }
}
