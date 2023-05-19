pub mod c1 {
    pub fn insertion_sort(arr: &mut Vec<f64>) {
        let mut i: i32;
        let mut value: f64;

        for j in 1..arr.len() {
            value = arr[j];
            i = (j - 1) as i32;

            while i >= 0 && arr[i as usize] > value {
                arr[(i+1) as usize] = arr[i as usize];

                i = i - 1;
            }
            arr[(i + 1) as usize] = value;
        }
    }

    pub fn merge_sort(arr: &mut Vec<f64>) {
        println!("{:?}", arr)
    }
}


#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod test_insertion_sort {
        use crate::c1::{insertion_sort};
        #[test]
        fn insertion_sort_test_without_sort() {
            let mut arr = vec![1.0, 5.0];
            insertion_sort(&mut arr);
            for i in 0..arr.len()-1 {
                assert_eq!(arr[i] < arr[i+1], true);
            }
        }
    
        #[test]
        fn insertion_sort_test_with_sort() {
            let mut arr = vec![5.0, 1.0];
            insertion_sort(&mut arr);
            assert_eq!(arr[0], 1.0);
            assert_eq!(arr[1], 5.0);
        }

        #[test]
        fn insertion_sort_test_with_four_numbers() {
            let mut arr = vec![5.0, 1.0, 9.0, 2.3];
            insertion_sort(&mut arr);
            assert_eq!(arr[0], 1.0);
            assert_eq!(arr[1], 2.3);
            assert_eq!(arr[2], 5.0);
            assert_eq!(arr[3], 9.0);
        }
    
        #[test]
        fn insertion_sort_test_with_many_numbers() {
            let mut arr = vec![1.0, 5.0, 3.0, 0., 2.1, 4.5, 2.6, 20.5, 8.3, 4.56, 3.2, 5.6, 9.04];
            insertion_sort(&mut arr);
            println!("Depois de ordenar {:?}", arr);
            for i in 0..arr.len()-1 {
                assert_eq!(arr[i] < arr[i+1], true);
            }
        }
    }
}