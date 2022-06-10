/// combination 알고리즘을 구현합니다.
///
/// arr: 조합에 사용할 배열
///
/// len: 조합할 갯수
///
/// f: 조합된 항목이 이 함수로 전달됩니다. true를 반환하면 해당 조합이 결과 리스트에 포함됩니다.
pub fn combinations<T, F>(arr: &[T], len: usize, mut f: F) -> Vec<Vec<&T>>
where
    F: FnMut(&Vec<&T>) -> bool,
{
    let mut result_list: Vec<Vec<&T>> = Vec::new();

    if arr.len() < len || len == 0 {
        return result_list;
    }

    let mut result: Vec<&T> = Vec::with_capacity(len);
    combinations_recur(arr, len, 0, &mut result, &mut f, &mut result_list);

    result_list
}

fn combinations_recur<'a, 'b, T, F>(
    arr: &'a [T],
    len: usize,
    start_position: usize,
    result: &mut Vec<&'a T>,
    f: &mut F,
    result_list: &'b mut Vec<Vec<&'a T>>,
) where
    F: FnMut(&Vec<&T>) -> bool,
{
    if len == 0 {
        if f(result) {
            let mut r_list: Vec<&T> = Vec::new();
            for r in result {
                r_list.push(r);
            }
            result_list.push(r_list);
        }
        return;
    }
    for i in start_position..=arr.len() - len {
        result.push(&arr[i]);
        combinations_recur(arr, len - 1, i + 1, result, f, result_list);
        result.pop();
    }
}

#[test]
fn combination_test() {
    let v = vec![1, 2, 3, 4, 5];

    let m = combinations(&v, 3, |_| true);
    for comb in m {
        let mut print_str: String = String::new();
        for c in comb {
            print_str.push_str(c.to_string().as_str());
            print_str.push_str(", ");
        }
        println!("{}", print_str);
    }
}
