// 对arr[low:high]进行partition操作，将第1个元素放在排序后的位置
fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pick first element as pivot
    let e = arr[low].clone();

    // arr[low+1:j] < v
    let mut j = low;
    // arr[j+1:i) > v
    let mut i = low + 1;
    while i <= high {
        if arr[i].clone() < e {
            j += 1;
            arr.swap(j, i);
        }
        i += 1;
    }

    arr.swap(low, j);
    j
}

fn quick_sort<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition(arr, low, high);
    println!("p = {}", p);
    if p > 0 {
        quick_sort(arr, low, p - 1);
    }
    quick_sort(arr, p + 1, high);
}

pub fn sort_v1<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort(arr, 0, high)
    }
}

fn integer() {
    let mut res = vec![4, 1, 8, 5, 7];
    sort_v1(&mut res);
    assert_eq!(res, vec![1, 4, 5, 7, 8]);
}

fn chars() {
    let mut res = vec!['A', 'a', 'h', 'b', 'W'];
    sort_v1(&mut res);
    assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
}

pub fn run() {
    integer();
    chars();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v1(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
    }

    #[test]
    fn chars() {
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        sort_v1(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
    }
}
