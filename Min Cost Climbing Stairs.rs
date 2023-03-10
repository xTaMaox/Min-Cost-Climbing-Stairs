use std::collections::HashMap;

impl Solution {
    fn dp(cost: &[i32], i: usize, memo: &mut HashMap<usize, i32>) -> i32 {
        if i >= cost.len() {
            0
        } else if let Some(min_cost) = memo.get(&i) {
            *min_cost
        } else {
            let rez = cost[i] + Self::dp(cost, i + 1, memo).min(Self::dp(cost, i + 2, memo));
            memo.insert(i, rez);
            rez
        }
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        Self::dp(&cost, 0, &mut memo);
        (*memo.get(&0).unwrap()).min(*memo.get(&1).unwrap())
    }
}
