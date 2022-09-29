pub fn bubble_sort<T: Ord>(num_slice: &mut [T]) -> &mut [T] {
    let last_position = num_slice.len() - 1;

    for i in 0..last_position {
        for j in 0..last_position - i {
            if num_slice[j + 1] <= num_slice[j] {
                // swap
                num_slice.swap(j, j + 1);
            }
        }
    }

    num_slice
}
