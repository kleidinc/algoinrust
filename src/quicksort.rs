fn quicksort<T: Ord>(slice: &mut [T]) {
    // if the slice is 0,1 or 2
    match slice.len() {
        1 | 0 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    // for any slice 3 or larger

    // Take the last element as the pivot
    let (pivot, rest) = slice
        .split_last_mut()
        .expect("The slice is not 0 or 1 elements");

    let mut left = 0;
    let mut right = rest.len() - 1;

    while right > left {
        // do stuff
        if &rest[left] <= pivot {
            // no swap needed
            left += 1;
        } else if &rest[right] >= pivot {
            // no swap needed
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    let pivot_index = slice.len() - 1;
    slice.swap(pivot_index, left);
    let (low, hi) = slice.split_at_mut(left);
    quicksort(low);
    quicksort(&mut hi[1..]);
}

#[test]
fn quicksort_works() {
    let mut thingie = vec![3, 2, 1];
    quicksort(&mut thingie);
    assert_eq!(thingie, &[1, 2, 3]);
}
