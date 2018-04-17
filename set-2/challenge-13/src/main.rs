use std::collections::BTreeMap;

type Profile = BTreeMap<String, String>;

fn main() {
    let profile1 = decode("foo=bar&baz=qux&zap=zazzle");

    let profile2 = profile_for("foo@bar.com");

    assert_eq!("baz=qux&foo=bar&zap=zazzle", encode(&profile1));
    assert_eq!("email=foo@bar.com&role=user&uid=10", encode(&profile2));
}

fn decode(input: &str) -> Profile {
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

fn encode(profile: &Profile) -> String {
    profile
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                k.replace("&", "").replace("=", ""),
                v.replace("&", "").replace("=", ""),
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn profile_for(email: &str) -> Profile {
    let mut map = Profile::new();
    map.insert("email".to_string(), email.to_string());
    map.insert("uid".to_string(), "10".to_string());
    map.insert("role".to_string(), "user".to_string());
    map
}
