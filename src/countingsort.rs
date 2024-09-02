// Countingsort
// [0,10,8,10]
//
fn countingsort(slice: &mut [i128]) {
    // 1. Get the max number in the slice
    println!("The slice input is {:?}", slice);
    let mut max = 0;
    for element in slice.iter() {
        if *element as usize > max {
            max = *element as usize;
            println!("The max has been updated: {}", max);
        }
    }
    // 2. Create a new array with max +1 amount of 0's
    // [0,0,0,0,0,0,0,0,0,0,11]
    let mut counts = vec![0; max + 1];
    println!("the counts array: {:?}", counts);
    // 3. Loop over slice and count each index [from 0 to max] and ++ for each occurrence
    // [1,0,0,0,0,0,0,0,1,0,2]
    for index in 0..max + 1 {
        for element in slice.iter() {
            if *element == index as i128 {
                // the index of a slice is of type usize
                counts[index] += 1;
                println!(
                    "The element {} and index{} is increased by 1:",
                    element, index
                );
            }
        }
    }

    // 4. Cummulate the values to reflect the start and end positions of each index values
    //
    // [1,0,0,0,0,0,0,0,1,0, 2 ]
    // [1,0, 4, 8, 9, 9, 9, 10, 10, 11]

    println!("The initial counts {:?}", counts);
    let mut position = counts.clone();
    //
    for index in 1..max + 1 {
        position[index] += position[index - 1];
    }
    println!(
        "The updated counts : {:?} with length {}",
        position,
        position.len()
    );
    //
    // 6. Repopulate the given slice in place by overwriting the values in the slice
    // with new values
    // Make a temporary copy of the input slice for reference, read that copied slice
    // and update the input slice
    // Reading should be done from end to start
    println!("the slice to sort: {:?}", slice);
    let mut slice_clone = vec![0; slice.len()];
    slice_clone[..].clone_from_slice(slice);
    println!("the slice clone is {:?}", slice_clone);
    //

    // We make a copy of the input slice so we can use it to read all elements
    // and update the input slice values
    //
    // We loop in reverse, because we want the sort to be stable
    //

    // the slice : [0,4,5,5,1] // the counts : [1,2,2,2,2,3,4]
    // element = 1
    // 0 .. 6
    // element = 1 / index = 1 => counts[1] = 2 // so we need to
    println!("The counts {:?} and position {:?}", counts, position);
    for element in slice_clone.iter().rev() {
        // check if element exists in the counts
        // if element is 0 and counts[element] > 0 => place in slice on position and decrease
        // count[element]
        // else if counts[element] - previous counts > 0 => place in slice on counts[elements]
        // position and decrease count

        if *element == 0 && position[*element as usize] > 0 {
            // This is an edge case because here the position doesnt need to be calculated
            //
            // In this specific case the position is actually equal to the count
            slice[position[*element as usize] - 1] = *element;
            println!(
                "Placing the 0 {} on position {}",
                element,
                position[*element as usize] - 1
            );
            position[*element as usize] -= 1;
            counts[*element as usize] -= 1;
            println!(
                "The updated slice is now : {:? } and counts : {:?} and position {:?}",
                slice, counts, position
            );
        } else if counts[*element as usize] > 0 {
            // there is an element like that
            // Here the position is actually correct > you dont need to substract the position
            println!(
                "Placing the element {} on position {}",
                element,
                position[*element as usize] - 1
            );
            slice[position[*element as usize] - 1] = *element;
            position[*element as usize] -= 1;
            counts[*element as usize] -= 1;
            println!("The updated slice is now: {:?}", slice);
        }
    }

    println!("The updated slice {:?}", slice);
}

#[test]
fn countingsort_works() {
    let mut slice: Vec<i128> = vec![0, 4, 5, 5, 1];
    countingsort(&mut slice);
    assert_eq!(slice, &[0, 1, 4, 5, 5]);
}
