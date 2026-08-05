impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // upper bound is max in the piles (using right exclusive -> max + 1)
        // lower bound is 1
        fn time_taken(piles: &Vec<i32>, k: i32) -> i32 {
            piles.iter().fold(0, |acc, &x| acc + (x + k - 1) / k)
        }

        let mut max = piles.iter().max().unwrap();
        let mut k_left = 1;
        let mut k_right = max + 1;
        while k_left < k_right {
            let k = k_left + (k_right-k_left)/2;
            if time_taken(&piles, k) > h {
                k_left = k + 1;
            } else {
                k_right = k;
            }
        }
        return k_left;
    }
}

