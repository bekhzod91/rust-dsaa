pub fn insertion_sort<T: std::cmp::PartialOrd + std::fmt::Display + std::fmt::Debug>(arr: &mut Vec<T>) {
    let length = arr.len();
    
    for i in 0..length {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_natural_number() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        insertion_sort(&mut array);

        assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_insertion_sort_zero() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        insertion_sort(&mut array);

        assert_eq!(array, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_insertion_sort_dublicate_number() {
        let mut array = vec![3, 3, 2, 0, 0, 1, 1];
        insertion_sort(&mut array);

        assert_eq!(array, vec![0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_insertion_sort_negative_number() {
        let mut array = vec![0, -1, -2, -3, -4, -5];
        insertion_sort(&mut array);

        assert_eq!(array, vec![-5, -4, -3, -2, -1, 0]);
    }

    #[test]
    fn test_insertion_sort_empty_array() {
        let mut array: Vec<i32> = vec![];
        insertion_sort(&mut array);

        assert_eq!(array, vec![]);
    }

    #[test]
    fn test_insertion_sort_one_element() {
        let mut array = vec![1];
        insertion_sort(&mut array);

        assert_eq!(array, vec![1]);
    }
}
