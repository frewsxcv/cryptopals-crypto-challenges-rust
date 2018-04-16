use std::collections::HashMap;

fn main() {
    println!("{:?}", parse("foo=bar&baz=qux&zap=zazzle"));
}

fn parse(input: &str) -> HashMap<String, String> {
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
