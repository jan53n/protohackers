use std::collections::BTreeMap;
use std::ops::Bound::Included;

pub type PriceStore = BTreeMap<i32, i32>;

pub fn get_mean_from_minmax_time(store: &PriceStore, min_time: i32, max_time: i32) -> i32 {
    let mut count: i64 = 0;
    let mut total: i64 = 0;

    if max_time < min_time {
        return 0;
    }

    let range = store.range((Included(min_time), Included(max_time)));

    for (_, price) in range {
        count += 1;
        total += *price as i64;
    }

    if count == 0 {
        return 0;
    }

    (total / count) as i32
}
