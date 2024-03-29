#[derive(Debug)]

struct Item {
    value: i32,
    starts: Vec<Vec<i32>>,
    ends: Vec<Vec<i32>>,
}

impl Item {
    pub fn new(value: i32, start: bool, v: Vec<i32>) -> Self {
        if start {
            Item {
                value: value,
                starts: vec![v],
                ends: vec![],
            }
        } else {
            Item {
                value: value,
                starts: vec![],
                ends: vec![v],
            }
        }
    }
}

impl Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Item> = HashMap::new();
        for v in intervals.iter() {
            map.entry(v[0])
                .and_modify(|i| i.starts.push(v.clone()))
                .or_insert(Item::new(v[0], true, v.clone()));

            map.entry(v[1])
                .and_modify(|i| i.ends.push(v.clone()))
                .or_insert(Item::new(v[1], false, v.clone()));
        }

        //println!("{:?}", map);

        let mut points: Vec<&Item> = map.values().collect();
        points.sort_by(|x, y| x.value.cmp(&y.value));

        //println!("{:?}", &points);

        let mut ret = Vec::new();
        if points.is_empty() {
            return ret;
        }

        let mut status = 0; // 0: expect start, 1: expept end
        let mut start = 0;
        let mut end = 0;

        for item in points.iter() {
            if !item.starts.is_empty() {
                if status == 0 {
                    start = item.value;

                    end = start;

                    status = 1;
                }

                let mut max = item.starts[0][1];

                for range in item.starts.iter() {
                    if range[1] > max {
                        max = range[1];
                    }
                }

                if max > end {
                    end = max;
                }
            }

            if !item.ends.is_empty() {
                // item.ends is not empty

                if item.value == end {
                    ret.push(vec![start, end]);

                    status = 0;
                }
            }
        }

        ret
    }

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;

        intervals.push(new_interval);

        return Solution::merge(intervals);
    }
}

pub struct Solution;

fn main() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![[1, 5], [6, 9]]
    );

    assert_eq!(
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![[1, 2], [3, 10], [12, 16]]
    );
}
