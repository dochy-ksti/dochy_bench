use crate::save_1gb_100::{save_1gb_bench::LOOP, save_1gb_json_bench::DataJson};
use dochy::error::DpResult;
use rand::{thread_rng, Rng};
use std::time::Instant;

#[allow(dead_code)]
//#[test]
pub(crate) fn load_json_bench() -> DpResult<()> {
    let json_dir = "src/save_1gb_100/dat_dir";
    let mut rng = thread_rng();
    let r = rng.gen_range(0..LOOP);

    let path = format!("{}/d{}.json", json_dir, r);
    let start_time = Instant::now();
    let json_str = std::fs::read_to_string(path)?;

    let d: DataJson = serde_json::from_str(&json_str).unwrap();

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} milliseconds", duration.as_millis());

    Ok(())
}
