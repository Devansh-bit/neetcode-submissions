impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.is_empty() {
            return 0;
        }
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort_unstable_by_key(|&(pos, _speed)| Reverse(pos));
        let mut fleets = 1;
        let mut t_max = (target - cars[0].0) as f32 / cars[0].1 as f32;
        for i in 1..cars.len() {
            let t_current = (target - cars[i].0) as f32 / cars[i].1 as f32;
            if t_current > t_max {
                fleets += 1;
                t_max = t_current;
            }  
        }
        fleets
    }
}
