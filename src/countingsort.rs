// Countingsort
// [0,10,8,10,1,1,3,3,3,3,2,5]
//
fn countingsort(slice: &mut [u128]) {
    // 1. Get the max number in the slice
    let mut max = 0;
    for element in slice.iter() {
        if *element > max {
            max = *element;
        }
    }
    // 2. Create a new array with max +1 amount of 0's
    // [0,0,0,0,0,0,0,0,0,0,0]
    let mut counts = [max + 1; 0];
    // 3. Loop over slice and count each index [from 0 to max] and ++ for each occurrence
    // [1, 2, 1, 4,1 ,0 ,0 ,1 ,0 ,1 ]
    for index in 0..max {
        for element in slice.iter() {
            if *element == index {
                // the index of a slice is of type usize
                counts[index as usize] += 1;
            }
        }
    }
    // 4. Cummulate the values to reflect the start and end positions of each index values
    //
    // [1, 2, 1, 4, 1, 0, 0 ,1 , 0 , 1 ]
    // [1, 3, 4, 8, 9, 9, 9, 10, 10, 11]
    for index in 1..max as usize {
        counts[index] = counts[index - 1];
    }
    // 5. Use the final array to create a new vector with the sorted values
    let mut sorted: Vec<u128> = Vec::new();
    // you need to loop over the counts and change the values
    let mut occurences;

    for index in 0..max {
        if index == 0 {
            occurences = counts[index as usize];
        } else {
            occurences = counts[index as usize] - counts[index as usize - 1];
        }
        while occurences >= 1 {
            sorted.push(index);
            occurences -= 1;
        }
    }

    // 6. Repopulate the given slice in place by overwriting the values in the slice
    // with new values
}

#[test]
fn countingsort_works() {
    let mut slice: Vec<u128> = vec![0, 10, 8, 10, 1, 1, 3, 3, 3, 3, 2, 5];
    countingsort(&mut slice);
    assert_eq!(slice, &[0, 1, 1, 2, 3, 3, 3, 3, 5, 9, 10]);
}
