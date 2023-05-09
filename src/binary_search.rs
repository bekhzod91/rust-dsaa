pub fn binary_search<T: std::cmp::PartialOrd>(arr: &Vec<T>, el: T) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        
        if arr[mid] == el {
            return Some(mid.try_into().unwrap());
        }

        if arr[mid] < el {
            left = mid + 1;
        }

        if arr[mid] > el {
            right = mid - 1;
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_binary_search_case_0() {
        let array: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(binary_search(&array, 0), Some(0));
        assert_eq!(binary_search(&array, 1), Some(1));
        assert_eq!(binary_search(&array, 2), Some(2));
        assert_eq!(binary_search(&array, 3), Some(3));
        assert_eq!(binary_search(&array, 4), Some(4));
        assert_eq!(binary_search(&array, 5), Some(5));
        assert_eq!(binary_search(&array, 6), Some(6));
        assert_eq!(binary_search(&array, 7), Some(7));
        assert_eq!(binary_search(&array, 8), Some(8));
        assert_eq!(binary_search(&array, 9), Some(9));
        assert_eq!(binary_search(&array, 10), Some(10)); 

        assert_eq!(binary_search(&array, 11), None); 
    }

//    fn test_binary_search_case_1() {
//        let array: Vec<i32> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
//
//        assert_eq!(binary_search(&array, 10), Some(0));
//        assert_eq!(binary_search(&array, 20), Some(1));
//        assert_eq!(binary_search(&array, 30), Some(2));
//        assert_eq!(binary_search(&array, 40), Some(3));
//        assert_eq!(binary_search(&array, 50), Some(4));
//        assert_eq!(binary_search(&array, 60), Some(5));
//        assert_eq!(binary_search(&array, 70), Some(6));
//        assert_eq!(binary_search(&array, 80), Some(7));
//        assert_eq!(binary_search(&array, 90), Some(8));
//        assert_eq!(binary_search(&array, 100), Some(9));
//
//        assert_eq!(binary_search(&array, 120), None); 
//    }

//    fn test_binary_search_with_duplicates() {
//        let array: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 7, 8];
//
//        assert_eq!(binary_search(&array, 0), Some(0));
//        assert_eq!(binary_search(&array, 1), Some(1));
//        assert_eq!(binary_search(&array, 2), Some(2));
//        assert_eq!(binary_search(&array, 3), Some(3));
//        assert_eq!(binary_search(&array, 4), Some(4));
//        assert_eq!(binary_search(&array, 5), Some(5));
//        assert_eq!(binary_search(&array, 6), Some(6));
//        assert_eq!(binary_search(&array, 7), Some(7));
//        assert_eq!(binary_search(&array, 7), Some(7));
//        assert_eq!(binary_search(&array, 8), Some(9));
//
//        assert_eq!(binary_search(&array, 20), None); 
//    }


    #[test]
    fn test_binary_search_with_empty_array() {
        let array: Vec<i32> = vec![];

        assert_eq!(binary_search(&array, 0), None);
    }
}
