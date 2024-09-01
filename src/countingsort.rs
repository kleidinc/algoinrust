// Countingsort
// [0,10,8,10,1,1,3,3,3,3,2,5]
//
fn countingsort(slice: &mut [i128]) {
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
    let mut sorted: Vec<i128> = Vec::new();
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
    // Make a temporary copy of the input slice for reference, read that copied slice
    // and update the input slice
    // Reading should be done from end to start
    let mut slice_copy: Vec<i128> = Vec::new();
    slice_copy.copy_from_slice(slice);
    for (_index, element) in slice_copy.iter().enumerate().rev() {
        // use the element as the index in counts
        if counts[*element as usize] > 0 {
            // take the position of the element from the counts index
            slice[*element as usize] = *element;
            // decrease the count of element by one
            counts[*element as usize] -= 1;
        }
    }
}

#[test]
fn countingsort_works() {
    let mut slice: Vec<i128> = vec![0, 10, 8, 10, 1, 1, 3, 3, 3, 3, 2, 5];
    countingsort(&mut slice);
    assert_eq!(slice, &[0, 1, 1, 2, 3, 3, 3, 3, 5, 9, 10]);
}
