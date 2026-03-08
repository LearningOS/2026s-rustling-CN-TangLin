// Comprehensive unit tests for iterators exercises
// This module contains extensive test coverage for iterator functionality,
// including edge cases, boundary conditions, and error scenarios.

#[cfg(test)]
mod iterators1_tests {
    // Tests for iterators1.rs - Basic iterator creation and iteration

    #[test]
    fn test_basic_iterator_creation() {
        let fruits = vec!["apple", "banana", "cherry"];
        let mut iter = fruits.iter();

        assert_eq!(iter.next(), Some(&"apple"));
        assert_eq!(iter.next(), Some(&"banana"));
        assert_eq!(iter.next(), Some(&"cherry"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_from_vector() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_iterator() {
        let empty: Vec<i32> = vec![];
        let mut iter = empty.iter();

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // Exhausted iterator remains None
    }

    #[test]
    fn test_single_element_iterator() {
        let single = vec![42];
        let mut iter = single.iter();

        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_strings() {
        let words = vec!["hello", "world", "rust"];
        let mut iter = words.iter();

        assert_eq!(iter.next(), Some(&"hello"));
        assert_eq!(iter.next(), Some(&"world"));
        assert_eq!(iter.next(), Some(&"rust"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_state_preservation() {
        let numbers = vec![10, 20, 30];
        let mut iter = numbers.iter();

        let first = iter.next();
        let second = iter.next();

        assert_eq!(first, Some(&10));
        assert_eq!(second, Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_clone_independence() {
        let numbers = vec![1, 2, 3];
        let mut iter1 = numbers.iter();
        let mut iter2 = iter1.clone();

        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter2.next(), Some(&1));

        iter1.next(); // Skip 2
        assert_eq!(iter1.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
    }

    #[test]
    fn test_iterator_with_duplicate_values() {
        let duplicates = vec![1, 1, 2, 2, 3, 3];
        let mut iter = duplicates.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_zero_values() {
        let zeros = vec![0, 0, 0];
        let mut iter = zeros.iter();

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_negative_numbers() {
        let negatives = vec![-1, -2, -3];
        let mut iter = negatives.iter();

        assert_eq!(iter.next(), Some(&-1));
        assert_eq!(iter.next(), Some(&-2));
        assert_eq!(iter.next(), Some(&-3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_mixed_values() {
        let mixed: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let mut iter = mixed.iter();

        assert_eq!(iter.next(), Some(&Some(1)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_count() {
        let numbers = vec![1, 2, 3, 4, 5];
        let iter = numbers.iter();

        assert_eq!(iter.count(), 5);
    }

    #[test]
    fn test_iterator_collect() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_iterator_find() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = numbers.iter().find(|&&x| x > 3);

        assert_eq!(result, Some(&4));
    }

    #[test]
    fn test_iterator_find_not_found() {
        let numbers = vec![1, 2, 3];
        let result = numbers.iter().find(|&&x| x > 10);

        assert_eq!(result, None);
    }

    #[test]
    fn test_iterator_any() {
        let numbers = vec![1, 2, 3, 4, 5];

        assert!(numbers.iter().any(|&x| x > 3));
        assert!(!numbers.iter().any(|&x| x > 10));
    }

    #[test]
    fn test_iterator_all() {
        let positives = vec![1, 2, 3, 4, 5];
        let mixed = vec![1, -2, 3];

        assert!(positives.iter().all(|&x| x > 0));
        assert!(!mixed.iter().all(|&x| x > 0));
    }
}

#[cfg(test)]
mod iterators2_tests {
    use super::super::iterators2;

    // Tests for iterators2.rs - String capitalization functions

    #[test]
    fn test_capitalize_first_basic() {
        assert_eq!(iterators2::capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_capitalize_first_uppercase() {
        assert_eq!(iterators2::capitalize_first("Hello"), "Hello");
    }

    #[test]
    fn test_capitalize_first_empty() {
        assert_eq!(iterators2::capitalize_first(""), "");
    }

    #[test]
    fn test_capitalize_first_single_char() {
        assert_eq!(iterators2::capitalize_first("a"), "A");
    }

    #[test]
    fn test_capitalize_first_single_uppercase() {
        assert_eq!(iterators2::capitalize_first("A"), "A");
    }

    #[test]
    fn test_capitalize_first_number() {
        assert_eq!(iterators2::capitalize_first("123"), "123");
    }

    #[test]
    fn test_capitalize_first_special_chars() {
        assert_eq!(iterators2::capitalize_first("@hello"), "@hello");
    }

    #[test]
    fn test_capitalize_first_mixed_case() {
        assert_eq!(iterators2::capitalize_first("hELLo"), "HELLo");
    }

    #[test]
    fn test_capitalize_first_unicode() {
        assert_eq!(iterators2::capitalize_first("éclair"), "Éclair");
    }

    #[test]
    fn test_capitalize_first_multibyte() {
        assert_eq!(iterators2::capitalize_first("中文"), "中文"); // Should not change non-ASCII
    }

    #[test]
    fn test_capitalize_words_vector_basic() {
        let words = vec!["hello", "world"];
        let result = iterators2::capitalize_words_vector(&words);
        assert_eq!(result, vec!["Hello", "World"]);
    }

    #[test]
    fn test_capitalize_words_vector_empty() {
        let words: Vec<&str> = vec![];
        let result = iterators2::capitalize_words_vector(&words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_capitalize_words_vector_single() {
        let words = vec!["hello"];
        let result = iterators2::capitalize_words_vector(&words);
        assert_eq!(result, vec!["Hello"]);
    }

    #[test]
    fn test_capitalize_words_vector_with_spaces() {
        let words = vec!["hello", " ", "world"];
        let result = iterators2::capitalize_words_vector(&words);
        assert_eq!(result, vec!["Hello", " ", "World"]);
    }

    #[test]
    fn test_capitalize_words_vector_mixed() {
        let words = vec!["hello", "123", "world"];
        let result = iterators2::capitalize_words_vector(&words);
        assert_eq!(result, vec!["Hello", "123", "World"]);
    }

    #[test]
    fn test_capitalize_words_vector_already_capitalized() {
        let words = vec!["Hello", "World"];
        let result = iterators2::capitalize_words_vector(&words);
        assert_eq!(result, vec!["Hello", "World"]);
    }

    #[test]
    fn test_capitalize_words_string_basic() {
        let words = vec!["hello", " ", "world"];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_capitalize_words_string_empty() {
        let words: Vec<&str> = vec![];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "");
    }

    #[test]
    fn test_capitalize_words_string_single() {
        let words = vec!["hello"];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_capitalize_words_string_with_punctuation() {
        let words = vec!["hello", ",", "world"];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "Hello,World");
    }

    #[test]
    fn test_capitalize_words_string_multiple_words() {
        let words = vec!["hello", " ", "rust", " ", "world"];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "Hello Rust World");
    }

    #[test]
    fn test_capitalize_words_string_with_numbers() {
        let words = vec!["test", "123"];
        let result = iterators2::capitalize_words_string(&words);
        assert_eq!(result, "Test123");
    }

    #[test]
    fn test_capitalize_first_preserves_rest() {
        assert_eq!(iterators2::capitalize_first("hello world"), "Hello world");
    }

    #[test]
    fn test_capitalize_first_with_newline() {
        assert_eq!(iterators2::capitalize_first("\nhello"), "\nhello");
    }

    #[test]
    fn test_capitalize_first_with_tab() {
        assert_eq!(iterators2::capitalize_first("\thello"), "\thello");
    }
}

#[cfg(test)]
mod iterators3_tests {
    use super::super::iterators3::{self, DivisionError, NotDivisibleError};

    // Tests for iterators3.rs - Division operations with Result types

    #[test]
    fn test_divide_success() {
        assert_eq!(iterators3::divide(10, 2), Ok(5));
        assert_eq!(iterators3::divide(81, 9), Ok(9));
        assert_eq!(iterators3::divide(100, 10), Ok(10));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(iterators3::divide(10, 0), Err(DivisionError::DivideByZero));
        assert_eq!(iterators3::divide(0, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_not_divisible() {
        assert_eq!(
            iterators3::divide(10, 3),
            Err(DivisionError::NotDivisible(NotDivisibleError { dividend: 10, divisor: 3 }))
        );
        assert_eq!(
            iterators3::divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError { dividend: 81, divisor: 6 }))
        );
    }

    #[test]
    fn test_divide_zero_by_number() {
        assert_eq!(iterators3::divide(0, 5), Ok(0));
        assert_eq!(iterators3::divide(0, 100), Ok(0));
    }

    #[test]
    fn test_divide_negative_numbers() {
        assert_eq!(iterators3::divide(-10, 2), Ok(-5));
        assert_eq!(iterators3::divide(10, -2), Ok(-5));
        assert_eq!(iterators3::divide(-10, -2), Ok(5));
    }

    #[test]
    fn test_divide_large_numbers() {
        assert_eq!(iterators3::divide(1000000, 1000), Ok(1000));
        assert_eq!(iterators3::divide(i32::MAX, 1), Ok(i32::MAX));
    }

    #[test]
    fn test_divide_one_by_itself() {
        assert_eq!(iterators3::divide(1, 1), Ok(1));
        assert_eq!(iterators3::divide(42, 42), Ok(1));
    }

    #[test]
    fn test_divide_result_with_list_success() {
        let result = iterators3::result_with_list();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 11, 1426, 3]);
    }

    #[test]
    fn test_divide_list_of_results_success() {
        let result = iterators3::list_of_results();
        assert_eq!(result.len(), 4);
        assert!(result.iter().all(|r| r.is_ok()));
    }

    #[test]
    fn test_not_divisible_error_structure() {
        let error = DivisionError::NotDivisible(NotDivisibleError {
            dividend: 10,
            divisor: 3,
        });
        assert_eq!(
            error,
            DivisionError::NotDivisible(NotDivisibleError { dividend: 10, divisor: 3 })
        );
    }

    #[test]
    fn test_divide_by_zero_error_structure() {
        assert_eq!(iterators3::divide(5, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_multiple_divisions_with_result() {
        let numbers = vec![10, 20, 30];
        let results: Vec<Result<i32, DivisionError>> = numbers
            .into_iter()
            .map(|n| iterators3::divide(n, 10))
            .collect();

        assert_eq!(results, vec![Ok(1), Ok(2), Ok(3)]);
    }

    #[test]
    fn test_division_with_remainder() {
        assert!(iterators3::divide(7, 3).is_err());
        assert!(iterators3::divide(15, 7).is_err());
    }

    #[test]
    fn test_divide_exact_multiples() {
        assert!(iterators3::divide(100, 25).is_ok());
        assert!(iterators3::divide(1000, 100).is_ok());
    }

    #[test]
    fn test_not_divisible_error_values() {
        let result = iterators3::divide(17, 5);
        if let Err(DivisionError::NotDivisible(err)) = result {
            assert_eq!(err.dividend, 17);
            assert_eq!(err.divisor, 5);
        } else {
            panic!("Expected NotDivisible error");
        }
    }

    #[test]
    fn test_divide_edge_cases() {
        // Maximum i32 divided by 2
        assert_eq!(iterators3::divide(i32::MAX, 2), Ok(i32::MAX / 2));

        // Minimum i32 divided by 2 (note: i32::MIN / 2 = -1073741824)
        assert_eq!(iterators3::divide(i32::MIN, 2), Ok(i32::MIN / 2));
    }

    #[test]
    fn test_divide_consistency() {
        // a / b = c should imply b * c = a (when divisible)
        let a = 100;
        let b = 25;
        if let Ok(c) = iterators3::divide(a, b) {
            assert_eq!(b * c, a);
        }
    }
}

#[cfg(test)]
mod iterators_advanced_tests {
    // Advanced iterator tests for comprehensive coverage

    #[test]
    fn test_iterator_take() {
        let numbers = vec![1, 2, 3, 4, 5];
        let taken: Vec<&i32> = numbers.iter().take(3).collect();

        assert_eq!(taken, vec![&1, &2, &3]);
    }

    #[test]
    fn test_iterator_skip() {
        let numbers = vec![1, 2, 3, 4, 5];
        let skipped: Vec<&i32> = numbers.iter().skip(2).collect();

        assert_eq!(skipped, vec![&3, &4, &5]);
    }

    #[test]
    fn test_iterator_filter() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

        assert_eq!(evens, vec![&2, &4, &6]);
    }

    #[test]
    fn test_iterator_chain() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();

        assert_eq!(chained, vec![&1, &2, &3, &4, &5, &6]);
    }

    #[test]
    fn test_iterator_flat_map() {
        let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let flattened: Vec<&i32> = nested.iter().flat_map(|x| x.iter()).collect();

        assert_eq!(flattened, vec![&1, &2, &3, &4, &5, &6]);
    }

    #[test]
    fn test_iterator_fold() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers.iter().fold(0, |acc, x| acc + x);

        assert_eq!(sum, 15);
    }

    #[test]
    fn test_iterator_reduce() {
        let numbers = vec![1, 2, 3, 4, 5];
        let product = numbers.iter().cloned().reduce(|acc, x| acc * x);

        assert_eq!(product, Some(120));
    }

    #[test]
    fn test_iterator_position() {
        let numbers = vec![1, 2, 3, 4, 5];
        let pos = numbers.iter().position(|&x| x == 3);

        assert_eq!(pos, Some(2));
    }

    #[test]
    fn test_iterator_nth() {
        let numbers = vec![10, 20, 30, 40, 50];
        let mut iter = numbers.iter();

        assert_eq!(iter.nth(2), Some(&30));
        assert_eq!(iter.nth(1), Some(&40));
    }

    #[test]
    fn test_iterator_last() {
        let numbers = vec![1, 2, 3, 4, 5];
        let last = numbers.iter().last();

        assert_eq!(last, Some(&5));
    }

    #[test]
    fn test_iterator_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();

        assert_eq!(sum, 15);
    }

    #[test]
    fn test_iterator_product() {
        let numbers = vec![1, 2, 3, 4];
        let product: i32 = numbers.iter().product();

        assert_eq!(product, 24);
    }

    #[test]
    fn test_iterator_max() {
        let numbers = vec![1, 5, 3, 9, 2];
        let max = numbers.iter().max();

        assert_eq!(max, Some(&9));
    }

    #[test]
    fn test_iterator_min() {
        let numbers = vec![5, 1, 9, 3, 2];
        let min = numbers.iter().min();

        assert_eq!(min, Some(&1));
    }

    #[test]
    fn test_iterator_max_empty() {
        let empty: Vec<i32> = vec![];
        let max = empty.iter().max();

        assert_eq!(max, None);
    }

    #[test]
    fn test_iterator_enumerate() {
        let words = vec!["apple", "banana", "cherry"];
        let enumerated: Vec<(usize, &&str)> = words.iter().enumerate().collect();

        assert_eq!(enumerated, vec![(0, &"apple"), (1, &"banana"), (2, &"cherry")]);
    }

    #[test]
    fn test_iterator_zip() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b", "c"];
        let zipped: Vec<(&i32, &&str)> = a.iter().zip(b.iter()).collect();

        assert_eq!(zipped, vec![(&1, &"a"), (&2, &"b"), (&3, &"c")]);
    }

    #[test]
    fn test_iterator_cycle() {
        let numbers = vec![1, 2, 3];
        let cycled: Vec<&i32> = numbers.iter().cycle().take(6).collect();

        assert_eq!(cycled, vec![&1, &2, &3, &1, &2, &3]);
    }

    #[test]
    fn test_iterator_map_while() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let mapped: Vec<i32> = numbers
            .iter()
            .map_while(|&x| if x < 4 { Some(x * 2) } else { None })
            .collect();

        assert_eq!(mapped, vec![2, 4, 6]);
    }

    #[test]
    fn test_iterator_try_fold() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result: Result<i32, &str> =
            numbers.iter().try_fold(0, |acc, &x| if x < 4 { Ok(acc + x) } else { Err("too large") });

        assert_eq!(result, Err("too large"));
    }

    #[test]
    fn test_iterator_peekable() {
        let numbers = vec![1, 2, 3];
        let mut iter = numbers.iter().peekable();

        assert_eq!(iter.peek(), Some(&&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.peek(), Some(&&2));
        assert_eq!(iter.next(), Some(&2));
    }

    #[test]
    fn test_iterator_scan() {
        let numbers = vec![1, 2, 3, 4, 5];
        let scanned: Vec<i32> = numbers.iter().scan(0, |state, &x| {
            *state += x;
            Some(*state)
        }).collect();

        assert_eq!(scanned, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_iterator_rev() {
        let numbers = vec![1, 2, 3, 4, 5];
        let reversed: Vec<&i32> = numbers.iter().rev().collect();

        assert_eq!(reversed, vec![&5, &4, &3, &2, &1]);
    }

    #[test]
    fn test_iterator_double_ended() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_cloned() {
        let numbers = vec![1, 2, 3];
        let cloned: Vec<i32> = numbers.iter().cloned().collect();

        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_copied() {
        let numbers = vec![1, 2, 3];
        let copied: Vec<i32> = numbers.iter().copied().collect();

        assert_eq!(copied, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_inspect() {
        let mut count = 0;
        let numbers = vec![1, 2, 3];
        let result: Vec<&i32> = numbers.iter().inspect(|_| count += 1).collect();

        assert_eq!(count, 3);
        assert_eq!(result, vec![&1, &2, &3]);
    }

    #[test]
    fn test_iterator_by_ref() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter();

        let first_three: Vec<&i32> = iter.by_ref().take(3).collect();
        let remaining: Vec<&i32> = iter.collect();

        assert_eq!(first_three, vec![&1, &2, &3]);
        assert_eq!(remaining, vec![&4, &5]);
    }

    #[test]
    fn test_iterator_partition() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds): (Vec<&i32>, Vec<&i32>) = numbers.iter().partition(|&&x| x % 2 == 0);

        assert_eq!(evens, vec![&2, &4, &6]);
        assert_eq!(odds, vec![&1, &3, &5]);
    }

    #[test]
    fn test_iterator_unzip() {
        let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let (numbers, chars): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

        assert_eq!(numbers, vec![1, 2, 3]);
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }
}

#[cfg(test)]
mod edge_case_tests {
    // Edge case tests for iterator behavior

    #[test]
    fn test_iterator_with_large_collection() {
        let large: Vec<i32> = (0..10000).collect();
        let mut iter = large.iter();

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(large.len(), 10000);
    }

    #[test]
    fn test_iterator_take_more_than_length() {
        let small = vec![1, 2, 3];
        let taken: Vec<&i32> = small.iter().take(10).collect();

        assert_eq!(taken, vec![&1, &2, &3]);
    }

    #[test]
    fn test_iterator_skip_more_than_length() {
        let small = vec![1, 2, 3];
        let skipped: Vec<&i32> = small.iter().skip(10).collect();

        assert!(skipped.is_empty());
    }

    #[test]
    fn test_iterator_with_special_chars() {
        let special = vec!["\n", "\t", "\r", " "];
        let mut iter = special.iter();

        assert_eq!(iter.next(), Some(&"\n"));
        assert_eq!(iter.next(), Some(&"\t"));
        assert_eq!(iter.next(), Some(&"\r"));
        assert_eq!(iter.next(), Some(&" "));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_bool() {
        let bools = vec![true, false, true, false];
        let mut iter = bools.iter();

        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), Some(&false));
        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), Some(&false));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_option() {
        let options = vec![Some(1), None, Some(3), None];
        let mut iter = options.iter();

        assert_eq!(iter.next(), Some(&Some(1)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), Some(&Some(3)));
        assert_eq!(iter.next(), Some(&None));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_result() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("error"), Ok(3)];
        let mut iter = results.iter();

        assert_eq!(iter.next(), Some(&Ok(1)));
        assert_eq!(iter.next(), Some(&Err("error")));
        assert_eq!(iter.next(), Some(&Ok(3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_with_float() {
        let floats = vec![1.0, 2.5, 3.14, 4.0];
        let mut iter = floats.iter();

        assert_eq!(iter.next(), Some(&1.0));
        assert_eq!(iter.next(), Some(&2.5));
        assert_eq!(iter.next(), Some(&3.14));
        assert_eq!(iter.next(), Some(&4.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_filter_map() {
        let numbers = vec!["1", "two", "3", "four", "5"];
        let parsed: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();

        assert_eq!(parsed, vec![1, 3, 5]);
    }

    #[test]
    fn test_iterator_for_each() {
        let mut sum = 0;
        let numbers = vec![1, 2, 3, 4, 5];

        numbers.iter().for_each(|x| sum += x);

        assert_eq!(sum, 15);
    }

    #[test]
    fn test_iterator_once() {
        use std::iter;
        let mut iter = iter::once(42);

        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_repeat() {
        use std::iter;
        let repeated: Vec<i32> = iter::repeat(5).take(3).collect();

        assert_eq!(repeated, vec![5, 5, 5]);
    }

    #[test]
    fn test_iterator_range() {
        let range = 0..5;
        let collected: Vec<i32> = range.collect();

        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_iterator_range_inclusive() {
        let range = 1..=5;
        let collected: Vec<i32> = range.collect();

        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iterator_size_hint() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter();

        assert_eq!(iter.size_hint(), (5, Some(5)));
        iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }

    #[test]
    fn test_iterator_into_iter() {
        let numbers = vec![1, 2, 3];
        let collected: Vec<i32> = numbers.into_iter().collect();

        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_from_iter() {
        let numbers: Vec<i32> = (0..5).collect();
        let collected: Vec<i32> = numbers.into_iter().collect();

        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }
}

#[cfg(test)]
mod performance_tests {
    // Performance and stress tests for iterators

    #[test]
    fn test_iterator_large_filter() {
        let large: Vec<i32> = (0..10000).collect();
        let filtered: Vec<&i32> = large.iter().filter(|&&x| x % 2 == 0).collect();

        assert_eq!(filtered.len(), 5000);
    }

    #[test]
    fn test_iterator_large_map() {
        let large: Vec<i32> = (0..10000).collect();
        let mapped: Vec<i32> = large.iter().map(|&x| x * 2).collect();

        assert_eq!(mapped.len(), 10000);
        assert_eq!(mapped[0], 0);
        assert_eq!(mapped[9999], 19998);
    }

    #[test]
    fn test_iterator_large_fold() {
        let large: Vec<i32> = (0..1000).collect();
        let sum: i32 = large.iter().fold(0, |acc, &x| acc + x);

        assert_eq!(sum, (0..1000).sum());
    }

    #[test]
    fn test_iterator_chain_large() {
        let a: Vec<i32> = (0..5000).collect();
        let b: Vec<i32> = (5000..10000).collect();
        let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();

        assert_eq!(chained.len(), 10000);
    }

    #[test]
    fn test_iterator_flat_map_large() {
        let nested: Vec<Vec<i32>> = (0..100).map(|i| (i*10..(i+1)*10).collect()).collect();
        let flattened: Vec<&i32> = nested.iter().flat_map(|x| x.iter()).collect();

        assert_eq!(flattened.len(), 1000);
    }

    #[test]
    fn test_iterator_cycle_performance() {
        let small = vec![1, 2, 3];
        let cycled: Vec<&i32> = small.iter().cycle().take(3000).collect();

        assert_eq!(cycled.len(), 3000);
    }

    #[test]
    fn test_iterator_zip_large() {
        let a: Vec<i32> = (0..5000).collect();
        let b: Vec<i32> = (5000..10000).collect();
        let zipped: Vec<(&i32, &i32)> = a.iter().zip(b.iter()).collect();

        assert_eq!(zipped.len(), 5000);
    }
}
