pub fn insertion(a: &mut Vec<i32>) {
    let len = a.len() as i32;

    for j in 1..len {
        let key = a[j as usize];

        let mut i = j - 1;
        while i >= 0 && a[i as usize] > key {
            a[(i + 1) as usize] = a[i as usize];
            i -= 1;
        }

        a[(i + 1) as usize] = key;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort() {
        let mut data = vec![5, 4, 2, 6, 1, 3];
        insertion(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }
}
