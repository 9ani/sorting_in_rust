mod sorting;
use sorting::{quick_sort, selection_sort, insertion_sort, merge_sort};
use std::cmp::Ordering;

fn main() {
    let mut nums = vec![4, 2, 5, 1, 3];
    quick_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Quick Sort: {:?}", nums);
    

    let mut nums = vec![4, 2, 5, 1, 3];
    selection_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Selection Sort: {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Insertion Sort: {:?}", nums);

    let mut nums = vec![4, 2, 5, 1, 3];
    merge_sort(&mut nums, &|a, b| a.cmp(b));
    println!("Merge Sort: {:?}", nums);
}
