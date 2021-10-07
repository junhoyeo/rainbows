#[cfg(test)]
mod tests {
    #[test]
    fn print_a_single_rainbow() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "1"]).assert();
        assert.success().stdout("Here is 1 rainbow for you!\n🌈\n");
    }

    #[test]
    fn print_two_rainbows() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "2"]).assert();
        assert
            .success()
            .stdout("Here are 2 rainbows for you!\n🌈🌈\n");
    }

    #[test]
    fn print_one_thousand_rainbows() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "1000"]).assert();
        assert.success().stdout(format!(
            "Here are 1000 rainbows for you!\n{}\n",
            "🌈".repeat(1000),
        ));
    }

    #[test]
    fn limit_number_of_rainbows_to_one_thousand() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "1001"]).assert();

        assert.success().stdout(format!(
            "Here are 1000 rainbows for you!\n{}\n",
            "🌈".repeat(1000),
        ));
    }

    #[test]
    #[allow(non_snake_case)]
    fn when_unparseable_argument_given__print_one_thousand_rainbows() {
        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "1ju43n8h9o0o5y4e6o"]).assert();

        assert.success().stdout(format!(
            "Here are 1000 rainbows for you!\n{}\n",
            "🌈".repeat(1000),
        ));
    }

    fn parse_number_of_rainbows(input: &str) -> Option<u16> {
        input
            .split(" ")
            .map(|s| s.to_string())
            .find(|s| s.parse::<f64>().is_ok())
            .and_then(|s| s.parse::<u16>().ok())
    }

    #[test]
    #[allow(non_snake_case)]
    fn when_no_argument_given__print_a_random_number_of_rainbows() {
        use std::convert::TryFrom;

        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.assert();

        let output: std::vec::Vec<u8> = assert.success().get_output().stdout.clone();
        let stdout = String::from_utf8(output).unwrap();
        assert!(stdout.contains("🌈"));

        let number_of_rainbows = parse_number_of_rainbows(&stdout).unwrap();
        let count_of_rainbows = u16::try_from(stdout.matches("🌈").count())
            .ok()
            .unwrap_or(0);
        assert_eq!(number_of_rainbows, count_of_rainbows);
    }

    #[test]
    #[allow(non_snake_case)]
    fn when_zero_given__print_a_random_number_of_rainbows() {
        use std::convert::TryFrom;

        let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let assert = cmd.args(["-n", "1000"]).assert();

        let output: std::vec::Vec<u8> = assert.success().get_output().stdout.clone();
        let stdout = String::from_utf8(output).unwrap();
        assert!(stdout.contains("🌈"));

        let number_of_rainbows = parse_number_of_rainbows(&stdout).unwrap();
        let count_of_rainbows = u16::try_from(stdout.matches("🌈").count())
            .ok()
            .unwrap_or(0);
        assert_eq!(number_of_rainbows, count_of_rainbows);
    }
}
