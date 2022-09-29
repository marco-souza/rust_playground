
pub fn merge_sort<T: Ord + Copy>(num_slice: &[T]) -> &[T] {
    if num_slice.len() <= 1 {
        return num_slice
    }

    match num_slice.len() {
        0 | 1 => num_slice,
        2 => {
            let [a, b]: [T] = *num_slice;
            match b < a {
                true => &[b, a],
                false => &[a, b],
            }
        },
        n => {
            let s1 = merge_sort(&num_slice[..n/2]);
            let s2 = merge_sort(&num_slice[n..]);

            return merge(s1, s2)
        }
    }
}

fn merge<I: Ord + Copy>(v1: &[I], v2: &[I]) -> &[I] {
    if v1.len() == 0 {return v2};
    if v2.len() == 0 {return v1};

    let [i1, s1 @ ..] = v1;
    let [i2, s2 @ ..] = v2;


    match i1 > i2 {
        true => &[&[i2, i1], merge(s1, s2)].concat(),
        false => &[&[i1, i2], merge(s1, s2)].concat(),
    }
}

