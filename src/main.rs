// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     a.sort_unstable();
//     a.dedup();
//     a
// }

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     let mut a_unique = Vec::new();
//     a.sort();
//     for item in a {
//         if a_unique.last() != Some(&item) {
//             a_unique.push(item);
//         }
//     }

//     a_unique
// }

// advanced 1: use generic types
// fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
//     let mut a_unique = Vec::new();
//     a.sort();
//     for item in a {
//         if a_unique.last() != Some(&item) {
//             a_unique.push(item);
//         }
//     }

//     a_unique
// }

// advanced 2: keep items in order
fn unique<T: Ord>(a: &mut [T]) -> Vec<T> {
    let mut a_unique = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for item in a.iter() {
        if seen.insert(item) {
            a_unique.push(*item);
        }
    }
    a_unique.sort();
    a_unique
}

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input: Vec<u32> = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input: Vec<u32> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
