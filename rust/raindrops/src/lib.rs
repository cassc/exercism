use std::collections::HashMap;
pub fn raindrops(n: u32) -> String {
    let mut m = HashMap::new();
    m.insert(3, "Pling");
    m.insert(5, "Plang");
    m.insert(7, "Plong");
    let mut s = String::from("");
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => (m.get(3), m.get(5), m.get(7)),
        (0, 0, _) => (m.get(3), m.get(5), ""),
        (0, _, _) => (m.get(3), "", ""),
        (_, 0, 0) => ("", m.get(5), m.get(7)),
        (_, _, 0) => ("", "", m.get(7)),
        (_, 0, _) => ("", m.get(5), ""),
        (_, _, _) => ("", "", ""),
    }

    s
}
