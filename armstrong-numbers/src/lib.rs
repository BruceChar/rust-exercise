pub fn is_armstrong_number(num: u32) -> bool {
    let mut nums = vec![];
    let mut nit = num;
    while nit != 0 {
        nums.push(nit % 10);
        nit /= 10;
    }
    let p = nums.len();
    let mut sum: u32 = 0;
    for n in nums {
        let tmp = sum.saturating_add(n.pow(p as u32));
        if tmp < sum {
            return false;
        } 
        sum = tmp;
    }
    sum == num
}
