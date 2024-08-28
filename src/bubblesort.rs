fn bubblesort<T: Ord>(slice: &mut [T]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..slice.len() - 1 {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                swapped = true
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut testlist = vec![9, 7, 3, 11];
    bubblesort(&mut testlist);
    assert_eq!(testlist, &[3, 7, 9, 11]);
}
