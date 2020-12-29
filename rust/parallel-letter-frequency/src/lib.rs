use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

fn frequency_of_words(input: &[String]) -> HashMap<char, usize> {
    let mut m = HashMap::new();

    input
        .iter()
        .flat_map(|s| s.chars())
        .filter(|c| c.is_alphabetic())
        .flat_map(char::to_lowercase)
        .for_each(|c| {
            let counter = m.entry(c).or_insert(0);
            *counter += 1;
        });
    m
}

fn merge_map(m1: &mut HashMap<char, usize>, m2: &HashMap<char, usize>) {
    for (k, v) in m2 {
        (*m1.entry(*k).or_insert(0)) += *v
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let len = input.len();
    let input: Vec<String> = input.iter().map(|s| String::from(*s)).collect();
    let size = len / worker_count;

    if size > 0 {
        let data = Arc::new(input);
        let mut results = Vec::with_capacity(worker_count);
        let mut i = 0;
        while i < len {
            let mut end = i + size;
            if end > len {
                end = len;
            }
            let input = Arc::clone(&data);
            results.push(thread::spawn(move || {
                frequency_of_words(&input[i..end])
            }));
            i = end;
        }

        let mut m = HashMap::new();
        for r in results {
            let mt = r.join().unwrap();
            merge_map(&mut m, &mt);
        }
        m
    } else {
        frequency_of_words(&input)
    }
}
