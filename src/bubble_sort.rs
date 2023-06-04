pub fn bubble_sort<T: std::cmp::PartialOrd>(arr: &mut Vec<T>) {
    let length = arr.len();

    for i in 0..length {
        let stop = length - i - 1;
        for j in 0..stop {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_natural_number() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut array);

        assert_eq!(array, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_zero() {
        let mut array = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubble_sort(&mut array);

        assert_eq!(array, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_dublicate_number() {
        let mut array = vec![3, 3, 2, 0, 0, 1, 1];
        bubble_sort(&mut array);

        assert_eq!(array, vec![0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_bubble_sort_negative_number() {
        let mut array = vec![0, -1, -2, -3, -4, -5];
        bubble_sort(&mut array);

        assert_eq!(array, vec![-5, -4, -3, -2, -1, 0]);
    }

    #[test]
    fn test_bubble_sort_empty_array() {
        let mut array: Vec<i32> = vec![];
        bubble_sort(&mut array);

        assert_eq!(array, vec![]);
    }

    #[test]
    fn test_bubble_sort_one_element() {
        let mut array = vec![1];
        bubble_sort(&mut array);

        assert_eq!(array, vec![1]);
    }
}
