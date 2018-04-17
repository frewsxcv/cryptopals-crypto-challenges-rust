use std::collections::HashMap;

type Profile = HashMap<String, String>;

fn main() {
    let profile1 = parse("foo=bar&baz=qux&zap=zazzle");
    println!("{:?}", profile1);
    let profile2 = profile_for("foo@bar.com");
    println!("{:?}", profile2);
}

fn parse(input: &str) -> Profile {
    input
        .split('&')
        .map(|pair| {
            let mut pair_iter = pair.split('=');
            let key = pair_iter.next().unwrap().to_owned();
            let value = pair_iter.next().unwrap().to_owned();
            (key, value)
        })
        .collect()
}

fn profile_for(email: &str) -> Profile {
    let mut map = HashMap::new();
    map.insert("email".to_string(), email.to_string());
    map.insert("uid".to_string(), "10".to_string());
    map.insert("role".to_string(), "user".to_string());
    map
}
