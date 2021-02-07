pub fn raindrops(n: u32) -> String {
    let pairs = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut r = String::new();
    pairs.iter().for_each(|(i, s)| {
        if n % i == 0 {
            r.push_str(s);
        }
    });
    if r.len() > 0 {
        r
    } else {
        n.to_string()
    }
}
