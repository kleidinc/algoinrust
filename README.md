# Algorithms in Rust

Implementing sorting algorithms in Rust. 

## std::slice methods to use 
- you can't increase the size of the slice // but you can take a slice of a slice
- you can, however, swap elements inside a slice using the **swap** method on slices
- you can split a slice with the **spit_at_mut** method into two slices


## Quicksort
The quicksort sorting algorithm uses divide and conquer approach (D&C). With quicksort you select a pivot element, and then divide the slice into two parts, one part containing all the elements lower than the pivot, and the right part containing all the elements higher than the pivot.

Then you run quicksort on the left and right separately, until the partitions become 1 element.

### Quicksort Termination condition
When you want to quicksort 0 or 1 element, you just can return.

### 2 elements you can also handle easier without having to choose a pivot. Just compare both elements and swap accordingly.

### So quicksort only makes sense if you have 3 elements. 
The smallest quicksort is with 3 elements.
1. you select a pivot
2. then compare the left element with the pivot 
3. then compare the right element with the pivot
4. swap accordingly


### Quicksort Pseudocode

if slice.len() = 0 => return
if slice.len() = 1 => return the element
if slice.len(2) => compare both and swap accordingly

the real algorith starts if there are at least 3 elements

choose first element as a pivot


