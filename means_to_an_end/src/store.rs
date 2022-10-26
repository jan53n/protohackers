use std::{cmp::max, collections::HashMap};

pub type PriceStore = HashMap<i32, i32>;

pub fn get_mean_from_minmax_time(store: &PriceStore, min_time: i32, max_time: i32) -> i32 {
    let mut c = 0;
    let mut t = 0;

    for i in min_time..(max_time + 1) {
        if let Some(v) = store.get(&i) {
            c += 1;
            t += v;
        }
    }

    t / max(c, 1)
}
