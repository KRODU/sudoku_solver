/// combination 알고리즘을 구현합니다.
///
/// arr: 조합에 사용할 배열
///
/// len: 조합할 갯수
///
/// f: 조합된 항목이 이 함수로 전달됩니다.
pub fn combinations<T, F>(arr: &[T], len: usize, mut f: F)
where
    F: FnMut(&Vec<&T>) -> bool,
{
    if arr.len() < len || len == 0 {
        return;
    }

    let mut result: Vec<&T> = Vec::with_capacity(len);
    combinations_recur(arr, len, 0, &mut result, &mut f);
}

fn combinations_recur<'a, T, F>(
    arr: &'a [T],
    len: usize,
    start_position: usize,
    result: &mut Vec<&'a T>,
    f: &mut F,
) -> bool
where
    F: FnMut(&Vec<&T>) -> bool,
{
    if len == 0 {
        return !f(result);
    }
    for i in start_position..=arr.len() - len {
        result.push(&arr[i]);
        if combinations_recur(arr, len - 1, i + 1, result, f) {
            return true;
        }
        result.pop();
    }

    false
}

#[test]
fn combination_test() {
    let v = vec![1, 2, 3, 4, 5];
    let mut print_str: String = String::new();

    combinations(&v, 3, |vec| {
        for c in vec {
            print_str.push_str(c.to_string().as_str());
            print_str.push_str(",");
        }
        print_str.pop();
        print_str.push_str("   ");
        true
    });

    assert_eq!(
        print_str,
        "1,2,3   1,2,4   1,2,5   1,3,4   1,3,5   1,4,5   2,3,4   2,3,5   2,4,5   3,4,5   "
    );

    print_str.clear();
    combinations(&v, 1, |vec| {
        for c in vec {
            print_str.push_str(c.to_string().as_str());
            print_str.push_str(",");
        }
        print_str.pop();
        print_str.push_str("   ");
        true
    });

    assert_eq!(print_str, "1   2   3   4   5   ");
}
