pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;

pub use bubblesort::BubbleSort;

// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn it_works() {
//         let mut unsorted = vec![9, 1, 5, 11];
//         crate::bubblesort::bubblesort(&mut unsorted);
//         assert_eq!(&unsorted, &[1, 5, 9, 11]);
//     }
// }
