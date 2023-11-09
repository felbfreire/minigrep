use minigrep::config::Config;
use minigrep::grep;


#[test]
fn test_minigrep_output() {

    let skipme = String::from("skipme");
    let file = String::from("poem.txt");
    let query = String::from("I");

    let args: [String; 3] = [skipme, query, file];

    let config = Config::build(args.into_iter());
    let result = grep(config.unwrap());

    let expected = "I'm nobody! Who are you?".to_string();

    assert_eq!(expected, result.unwrap());

}

