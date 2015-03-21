use std::collections::HashSet;

#[test]
fn test_times_two_vec() {
    let actual: Vec<i32> = gen![x*2 => x in [1, 2, 3, 4]];
    let expected = vec![2, 4, 6, 8];

    assert_eq!(actual, expected);
}

#[test]
fn test_no_change_vec() {
    let actual: Vec<i32> = gen![x => x in [1, 2, 3, 4]];
    let expected = vec![1, 2, 3, 4];

    assert_eq!(actual, expected);
}

#[test]
fn test_times_two_set() {
    let actual: HashSet<i32> = gen![x*2 => x in [1, 2, 3, 4]];
    let mut expected = HashSet::new();
    expected.insert(2);
    expected.insert(4);
    expected.insert(6);
    expected.insert(8);

    assert_eq!(actual, expected);
}

#[test]
fn test_no_change_set() {
    let actual: HashSet<i32> = gen![x => x in [1, 2, 3, 4]];
    let mut expected = HashSet::new();
    expected.insert(1);
    expected.insert(2);
    expected.insert(3);
    expected.insert(4);

    assert_eq!(actual, expected);
}

#[test]
fn test_chained_vec() {
    let initial: Vec<i32> = gen![x*2 => x in [1, 2, 3, 4]];
    let actual: Vec<i32> = gen![x*2 => x in initial];
    let expected = vec![4, 8, 12, 16];

    assert_eq!(actual, expected);
}

#[test]
fn test_same_var_name() {
    let x: Vec<i32> = gen![x*2 => x in [1, 2, 3, 4]];
    let actual: Vec<i32> = gen![x*2 => x in x];
    let expected = vec![4, 8, 12, 16];

    assert_eq!(actual, expected);
}

#[test]
fn test_tuple_vec() {
    let actual: Vec<(String,i32)> = gen![(format!("{}", x),x) => x in [1, 2, 3, 4]];
    let expected = vec![("1".to_string(),1),("2".to_string(),2),("3".to_string(),3),("4".to_string(),4)];

    assert_eq!(actual, expected);
}

#[test]
fn test_function_call() {
    fn double(x: i32) -> i32 {
        x * 2
    }

    let actual: Vec<i32> = gen![double(x) => x in [1, 2, 3, 4]];
    let expected = vec![2, 4, 6, 8];

    assert_eq!(actual, expected);
}
