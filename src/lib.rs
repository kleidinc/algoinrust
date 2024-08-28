use std::fmt;

mod bubblesort;
mod quicksort;

pub struct Displayable<T>([T]);

impl<T: fmt::Display> fmt::Display for Displayable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (index, element) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, " ,")?;
            }
            write!(f, "{}", element)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::bubblesort::*;

    #[test]
    fn it_works() {
        let mut unsorted = vec![9, 1, 5];
        bubblesort(&mut unsorted);
        assert_eq!(&unsorted, &[1, 5, 9]);
    }
}
