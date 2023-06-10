pub mod driverstation;

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};

    use crate::driverstation::TeamNumber;

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
