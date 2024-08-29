fn bubblesort<T: Ord + std::fmt::Display + std::fmt::Debug>(slice: &mut [T]) {
    // loop over the slice and compare the current element with the next
    // if the current is lower than the next -> do nochting
    // if the current is larger then the next, swap them
    // if swapped set the slice_swapped to true
    // loop until the slice_swapped is false, which
    let mut slice_swapped: bool;
    loop {
        slice_swapped = false;
        for i in 0..slice.len() {
            if i == slice.len() - 1 {
                continue;
            }
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                slice_swapped = true;
            }
        }
        // termination condition - means the bubblesort is sorted
        if !slice_swapped {
            break;
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut to_sort = vec![11, 9, 1, 22, 5];
    bubblesort(&mut to_sort);
    assert_eq!(to_sort, &[1, 5, 9, 11, 22]);
}
