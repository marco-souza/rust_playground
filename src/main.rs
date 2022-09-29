mod sorting;

// use sorting::bubble_sort::bubble_sort;
use sorting::merge_sort::merge_sort;

fn main() {
    let test_cases: &mut [&mut [i32]] = &mut [
        &mut [1, 3, 5, 12, 5, 4123, 6234, 123, 123, 191823],
        &mut [1, 3, 5, 12, 5, 4123, 6234, 123, 123],
        &mut [10, 12, 23, 1, 2, 3, 4, 5],
    ];

    for test_case in test_cases {
        println!("Test case: {:#?}", test_case);

        let sorted_array = merge_sort(test_case);

        println!("Sorted array: {:#?}", &sorted_array);
    }
}
