fn slug_issue(number: &str, name: &str) -> String {
    let name: String = name
        .to_owned()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    format!("{}-{}", number, str::replace(&name, " ", "-"))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./slug <issue_number> <issue_name>");
        println!("Example: ./slug 1 \"Sender Email Name should be same as App Name\"");
        println!("1-sender-email-name-should-be-same-as-app-name");
        std::process::exit(1);
    }

    let slug = slug_issue(&args[1], &args[2]);
    println!("{}", slug);
}

#[cfg(test)]
mod tests {

    use ::*;

    #[test]
    fn slugs_number_and_name() {
        let number = "1";
        let name = "Sender Email Name should be same as App Name";
        let expected = "1-sender-email-name-should-be-same-as-app-name";

        let actual = slug_issue(number, name);

        assert_eq!(expected, actual);
    }

}
