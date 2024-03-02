// takes the array and target as input, returns an Option type
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    // define the left and right indices of the array
    let mut left = 0;
    let mut right = arr.len();

    // loops until the left index is greater than the right
    while left < right {
        // finds the middle of the array by dividing and conquering
        let mid = left + (right - left) / 2;

        // if the target is the middle, return the middle
        // otherwise iterate from mid to left and mid to right
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // if the target is not in the array, return none
    None
}

fn main() {
    // define the array and the target
    let arr = [1, 3, 5, 7, 9];
    let target = 5;

    // return the results
    match binary_search(&arr, target) {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
}
