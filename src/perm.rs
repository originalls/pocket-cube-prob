use std::ops::{Add, AddAssign, DivAssign, MulAssign, SubAssign};

pub fn index_arr_to_id<T>(index_arr: &Vec<T>) -> T
where
    T: num_traits::PrimInt + Add<Output = T> + AddAssign<T> + SubAssign<T> + MulAssign<T> + Copy,
{
    let mut sum = T::zero();
    let mut mul = T::from(index_arr.len()).unwrap();
    for i in index_arr.iter() {
        // last index is not necessary, but this way the code is cleaner
        sum *= mul;
        sum += *i;
        mul -= T::one();
    }
    sum
}

pub fn id_to_index_arr<T>(id: &T, length: &T) -> Vec<T>
where
    T: num_traits::PrimInt + SubAssign<T> + DivAssign<T> + AddAssign<T> + Copy,
{
    let mut arr: Vec<T> = Vec::new();
    let mut id = *id;
    let mut mul = T::one();

    while mul <= *length {
        let rem = id % mul;
        id /= mul;
        arr.push(rem);
        mul += T::one();
    }
    arr.reverse();
    arr
}

pub fn permutation_to_index_arr<T>(
    identity_arr: &Vec<T>,
    permutated_arr: &Vec<T>,
    id_func: impl Fn(&T) -> u32,
) -> Vec<u32>
where
    T: Clone,
{
    if identity_arr.len() != permutated_arr.len() {
        panic!("Permutated vector length does not match identity vector length");
    }
    let mut identity_copy = identity_arr.clone();
    let mut index_arr: Vec<u32> = Vec::new();
    for i in permutated_arr.iter() {
        // last index is not necessary, but this way the code is cleaner
        let index = identity_copy
            .iter()
            .position(|r| id_func(r) == id_func(i))
            .unwrap();
        identity_copy.remove(index);
        index_arr.push(index as u32);
    }

    index_arr
}

pub fn index_arr_to_permutation<T>(identity_arr: &Vec<T>, index_arr: &Vec<u32>) -> Vec<T>
where
    T: Copy,
{
    let mut arr: Vec<T> = Vec::new();
    for i in 0..index_arr.len() {
        let j = identity_arr.len() - 1 - i;
        let index = index_arr[j];
        let elm = identity_arr[j];
        arr.insert(index as usize, elm);
    }

    arr
}

pub fn smushed_to_array<T, const N: usize>(id: u32, elm_func: impl Fn(u32) -> T) -> [T; N] {
    let mut id = id;
    let mut elms: Vec<T> = Vec::new();
    for _ in 0..N {
        let elm: T = elm_func(id % 3);
        elms.push(elm);
        id /= 3;
    }
    while elms.len() < N {
        elms.insert(0, elm_func(0));
    }

    elms.reverse();
    elms.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a unsmushed Vec of length {} but it was {}",
            N,
            v.len()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_pos() {
        let pc = PocketCube::new();
        assert_eq!(pc.get_pos_id().get_id(), 0);
    }
    #[test]
    fn test_identity_rot() {
        let pc = PocketCube::new();
        assert_eq!(pc.get_rot_id().get_id(), 0);
    }
}
