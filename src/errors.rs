#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::from("<unknown>"),
        }
    }
}

pub fn result_type() {
    let correct_json = r#"{"name": "Dodko Dodovič"}"#;
    let first = serde_json::from_str::<Person>(correct_json);
    println!("first = {:?}", first);

    let incorrect_json = r#"{"name": "Kokos, celé zle",}"#;
    let second = serde_json::from_str::<Person>(incorrect_json);
    println!("second = {:?}", second);
}

pub fn getting_result() {
    let first = serde_json::from_str::<Person>(r#"{"name": "Dodko Dodovič"}"#);
    let first_inner = first.unwrap_or_default();
    println!("first's name is {:?}", first_inner.name);

    let second = serde_json::from_str::<Person>(r#"{"a": 0}"#);
    let second_inner = second.unwrap_or_default();
    println!("second's name is {:?}", second_inner.name);
}

pub fn option_type() {
    let nonempty_list = vec!['a', 'b', 'c'];
    println!("nonempty_list's last is: {:?}", nonempty_list.last());

    let empty_list: Vec<char> = vec![];
    println!("empty_list's last is: {:?}", empty_list.last());
}
