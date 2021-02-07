use crossbeam::thread;
use std::collections::HashMap;

fn frequency_of_words(input: &[&str]) -> HashMap<char, usize> {
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
    let size = len / worker_count;

    if size > 0 {
        thread::scope(|s| {
            let mut results = Vec::with_capacity(worker_count);
            input.chunks(size).for_each(|chunk| {
                results.push(s.spawn(move |_| frequency_of_words(chunk)));
            });

            let mut m = HashMap::new();

            for r in results {
                let mt = r.join().unwrap();
                merge_map(&mut m, &mt);
            }
            m
        })
        .unwrap()
    } else {
        frequency_of_words(&input)
    }
}
