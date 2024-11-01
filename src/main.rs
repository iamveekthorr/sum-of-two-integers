fn main() {
    println!("Hello, world!");

    // Problem Statement:
    // Given a sorted array of unique integers and a target integer, return true if there exists a pair of numbers that sum to target, false otherwise.

    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 11;

    sum_of_two_integers(&arr, target);
}

fn sum_of_two_integers(arr: &[i32], target: i32) -> bool {
    let mut start_index = 0;
    let mut end_index = arr.len() - 1;

    while start_index < end_index {
        let sum = arr[start_index] + arr[end_index];

        if sum == target {
            return true;
        } else if sum < target {
            start_index += 1;
        } else {
            end_index -= 1;
        }
    }

    return false;
}
