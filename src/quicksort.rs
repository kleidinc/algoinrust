// What is the base case?
//
// What is the method to split up a slice in tow mutable parts?
//
//

fn quicksort<T: Ord>(slice: &mut [T]) {
    // 1. get the last element in the slice at the pivot
    if let Some((pivot, rest)) = slice.split_last_mut() {
        // now we have a pivot
        // and a separate slice rest we can operate on
    }
}

#[test]
fn quicksort_works() {}
