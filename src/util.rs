pub fn combinations<T, F>(arr: &[T], len: usize, f: F) -> Vec<Vec<&T>>
where
    F: Fn(&Vec<&T>) -> bool,
{
    let mut result_list: Vec<Vec<&T>> = Vec::new();

    if arr.len() < len {
        return result_list;
    }

    let mut result: Vec<&T> = Vec::with_capacity(len);
    combinations_recur(arr, len, 0, &mut result, &f, &mut result_list);

    result_list
}

fn combinations_recur<'a, 'b, T, F>(
    arr: &'a [T],
    len: usize,
    start_position: usize,
    result: &mut Vec<&'a T>,
    f: &F,
    result_list: &'b mut Vec<Vec<&'a T>>,
) where
    F: Fn(&Vec<&T>) -> bool,
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
