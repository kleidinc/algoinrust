// compare two elements
// swap the elements if the previous is larger than the following
// The sorting should be implemented in place
pub fn bubblesort<T: Ord + Eq>(slice: &mut [T]) {
    let mut swapped: bool;
    loop {
        swapped = false;
        for i in 0..slice.len() {
            while i < slice.len() + 1 {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        if !swapped {
            break;
        }
    }
}
