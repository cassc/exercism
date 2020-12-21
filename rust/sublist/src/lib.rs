#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let first_len = first_list.len();
    let second_len = second_list.len();

    if first_len >= second_len {
        return false;
    }

    let mut skip = 0;
    while skip + first_len <= second_len {
        if second_list[skip..skip + first_len] == *first_list {
            return true;
        }
        skip += 1;
    }

    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if *first_list == *second_list {
        return Comparison::Equal;
    }

    if sublist_of(first_list, second_list) {
        return Comparison::Sublist;
    }

    if sublist_of(second_list, first_list) {
        return Comparison::Superlist;
    }

    return Comparison::Unequal;
}
