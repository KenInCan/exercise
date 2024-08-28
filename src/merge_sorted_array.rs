pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if m == 0 {
        nums1.clone_from(nums2);
        return;
    }

    for _ in 0..(nums1.len() as i32 - m) {
        _ = nums1.pop();
    }

    let mut temp: Vec<i32> = Vec::new();
    while let Some(num2) = nums2.pop(){
        let num1 = nums1.pop().unwrap();
        if num1 < num2 {
            nums1.push(num1);
            temp.push(num2);
        } else {
            nums2.push(num2);
            temp.push(num1);
        }
    }
    while let Some(temp_num) = temp.pop() {
        nums1.push(temp_num);
    }
}

#[test]
fn should_merge() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1,2,2,3,5,6])
}

#[test]
fn should_output_1_when_nums1_1() {
    let mut nums1 = vec![1];
    let mut nums2: Vec<i32> = Vec::new();
    let m = 1;
    let n = 0;

    merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1])
}

#[test]
fn should_output_1_when_nums2_1() {
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2 = vec![1];
    let m = 0;
    let n = 1;

    merge(&mut nums1, m, &mut nums2, n);

    assert_eq!(nums1, vec![1])
}
