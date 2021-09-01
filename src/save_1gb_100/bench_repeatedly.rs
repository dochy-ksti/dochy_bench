use crate::save_1gb_100::save_1gb_bench::save_1gb_bench;

use super::{
    save_1gb_bench::{LOOP, VEC_SIZE},
    save_1gb_json_bench::save_1gb_json_bench,
    save_1gb_json_impl::save_1gb_json_impl,
};

const REPEAT: usize = 3;
#[allow(dead_code)]
//#[test]
fn bench_repeatedly() {
    for _ in 0..REPEAT {
        save_1gb_bench().unwrap();
    }
    // for _ in 0..REPEAT {
    //     save_1gb_json_bench().unwrap();
    // }
    for _ in 0..REPEAT {
        save_1gb_json_impl(VEC_SIZE, LOOP).unwrap();
    }
    for _ in 0..REPEAT {
        save_1gb_json_impl(VEC_SIZE / 100 * 17, LOOP).unwrap();
    }
}

// 712 milliseconds
// sum of file sizes 173005190
// 743 milliseconds
// sum of file sizes 175005172
// 710 milliseconds
// sum of file sizes 175005355
// 1909 milliseconds
// sum of file sizes 1021406031
// 2025 milliseconds
// sum of file sizes 1021406070
// 1882 milliseconds
// sum of file sizes 1021406213
// 366 milliseconds
// sum of file sizes 173611903
// 355 milliseconds
// sum of file sizes 173612125
// 356 milliseconds
// sum of file sizes 173612107

