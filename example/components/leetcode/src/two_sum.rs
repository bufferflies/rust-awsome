use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (i, value) in nums.into_iter().enumerate() {
        if let Some(&k) = map.get(&(target - value)) {
            if k != i {
                return vec![k as i32, i as i32];
            }
        }
        map.insert(value, i);
    }
    panic!("not found")
}

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    print!("aaq");
    for (index, v1) in nums.clone().into_iter().enumerate() {
        for i in index + 1..len {
            if let Some(&v2) = nums.get(i) {
                if v1 != v2 && v1 + v2 == target {
                    return vec![v1, v2];
                }
            }
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_print_codec() {
        let data = vec![1, 2];
        //    let mut ret=super::two_sum(data.clone(),3);
        //    println!("result:{:?}",ret);
        let ret = super::two_sum_2(data, 3);
        println!("result:{:?}", ret);
        assert_eq!(ret.len(), 2);
    }
}
