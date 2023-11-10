use minigrep::config::Config;
use minigrep::grep;


#[test]
fn test_minigrep_output() {

    let skipme = String::from("skipme");
    let file = String::from("poem.txt");
    let query = String::from("How");

    let args: [String; 3] = [skipme, query, file];

    let config = Config::build(args.into_iter());
    let result = grep(config.unwrap()).unwrap();

    let expected_1 = "How dreary to be somebody!".to_string();
    let expected_2 = "How public, like a frog".to_string();

    assert_eq!(result.len(), 2);
    assert_eq!(expected_1, result[0]);
    assert_eq!(expected_2, result[1]);
}

