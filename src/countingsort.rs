// unsorted = [9,8,1,3,3,4,9,2,2,2,4]
// 1. get the maximum of the numbers in the array
// max = 9
// 2. create an array (max+1) to hold the amount of occurrences of each number in the array
// count = [0,0,0,0,0,0,0,0,0]
// 3. Loop over the inputslice and count each occurrence and put it in the right location += 1
// count = [1,3,2,2,0,0,0,1,1]
// 4. Recalculate the occurences list // cummulative
// count = [1,4,6,8,9,10]
// 5. Build the sorted list

fn countingsort(slice: &mut [u128]) {
    let mut max = 0;
    // get the max
    for element in slice.iter() {
        if *element > max {
            max = *element;
        }
    }
    // build the counting vec
    let mut counting = [max + 1; 0];

    // populate the counting vec
    for index in 0..max {
        // loop from 0 to max
        for element in slice.iter() {
            if *element == index {
                // increase the value on position index in the counting vec
                counting[index as usize] += 1;
            }
        }
    }

    // change the counting vec to contain the start position of each element in the sorting slice
    for i in 0..max as usize {
        if i == max as usize {
            break;
        }
        counting[i] += counting[i + 1];
    }
    // // change the slice you received with the sorted values
    // for i in 0..max as usize {
    //     for element in slice.iter_mut() {
    //         //
    //         i;
    //     }
    // }
}

#[test]
fn countingsort_works() {
    let mut thingie = vec![1, 99, 10, 5, 7];
    countingsort(&mut thingie);
    assert_eq!(thingie, &[1, 5, 7, 10, 99]);
}
