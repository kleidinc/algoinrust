use super::*;
// compare two elements
// swap the elements if the previous is larger than the following
// The sorting should be implemented in place
pub fn bubblesort<T: fmt::Display + Ord>(slice: &mut [T]) {
    let mut _swapped: bool = false;
    loop {
        for i in 0..slice.len() {
            if i < slice.len() && slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
            }
        }
    }
}
