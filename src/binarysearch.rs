// The result should be a Result (Some or None)
fn binary_seach(slice: &[u128], search_item: u128) -> Option<usize> {
    let mut start_index: usize = 0;
    let mut end_index: usize = slice.len() - 1;
    let mut middle_index: usize;

    loop {
        middle_index = start_index + end_index / 2;

        if slice[middle_index] == search_item {
            // You found it, return the index!
            return Some(middle_index);
        } else if slice[middle_index - 1] == search_item {
            return Some(middle_index - 1);
        } else if slice[middle_index + 1] == search_item {
            return Some(middle_index + 1);
        } else if slice[middle_index - 1] < search_item || slice[middle_index + 1] < search_item {
            // You can discard the left
            start_index = middle_index + 1;
        } else if slice[middle_index - 1] > search_item {
            end_index = middle_index - 1;
        } else {
            // The element is not found in the slice
            return None;
        }

        if end_index - start_index == 0 {
            return None;
        }
    }
}

#[test]
fn binary_search_works() {
    let thingie = vec![1, 2, 4, 9, 11, 15, 27, 35];
    let result = binary_seach(&thingie, 2);
    assert_eq!(result, Some(1));
    assert_eq!(binary_seach(&thingie, 1), Some(0));
    assert_eq!(binary_seach(&thingie, 3), None);
    assert_eq!(binary_seach(&thingie, 35), Some(7));
}
