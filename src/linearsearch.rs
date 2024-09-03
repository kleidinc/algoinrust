// Objective: Write a linear_search function that loops through the
// values in an integer vector looking for a target value. it should
// return the index of that value if it appears in the vector, and
// -1 if the value is not present. It should also return the number
// of items in the vector that the function examined.
//
// Use the following signature:
// fn linear_search(vec: &Vec<i32>, target: i32) -> (i32,i32) {}

fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    //
    (1, 1)
}

#[test]
fn linear_search_works() {
    let vec = vec![9, 1, 2, 8, 7, 11];
    let result = linear_search(&vec, 1);
    assert_eq!(result, (1, 6));
    let result2 = linear_search(&vec, 99);
    assert_eq!(result2, (-1, 6));
}
