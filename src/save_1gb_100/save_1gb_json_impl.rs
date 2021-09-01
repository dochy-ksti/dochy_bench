use crate::save_1gb_100::save_1gb_bench::{gen_ascii};
use crate::util::get_file_lens;
use dochy::error::DpResult;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct DataJson {
    data0: String,
    data1: String,
    data2: String,
    data3: String,
    data4: String,
    data5: String,
    data6: String,
    data7: String,
    data8: String,
    data9: String,
}

pub(crate) fn save_1gb_json_impl(vec_size: usize, loop_count: usize) -> DpResult<()> {
    let json_dir = "src/save_1gb_100/dat_dir";
    std::fs::remove_dir_all(json_dir).ok();
    std::fs::create_dir(json_dir).ok();

    let mut d = init(vec_size);
    let start_time = Instant::now();
    for i in 0..loop_count {
        modify(&mut d);
        let s = serde_json::to_string(&d).unwrap();
        let mut file = std::fs::File::create(format!("{}/d{}.json", json_dir, i))?;
        file.write_all(s.as_bytes())?;
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} milliseconds", duration.as_millis());

    let lens = get_file_lens(json_dir, "json")?;
    let sum_files = lens.iter().fold(0, |sum, fl| sum + fl.len());
    println!("sum of file sizes {}", sum_files);
    for fl in &lens {
        //println!("file {} len {}", fl.file_name(), fl.len())
    }
    Ok(())
}

fn modify(d: &mut DataJson) {
    let mut rng = rand::thread_rng();
    let r: usize = rng.gen_range(0..10);
    match r {
        0 => modify_str(&mut d.data0),
        1 => modify_str(&mut d.data1),
        2 => modify_str(&mut d.data2),
        3 => modify_str(&mut d.data3),
        4 => modify_str(&mut d.data4),
        5 => modify_str(&mut d.data5),
        6 => modify_str(&mut d.data6),
        7 => modify_str(&mut d.data7),
        8 => modify_str(&mut d.data8),
        9 => modify_str(&mut d.data9),
        _ => unreachable!(),
    }
}

fn modify_str(s: &mut String) {
    let s = unsafe { s.as_bytes_mut() };
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..s.len());
    s[i] = gen_ascii(&mut rng);
}

fn init(vec_size : usize) -> DataJson {
    DataJson {
        data0: random_str(vec_size),
        data1: random_str(vec_size),
        data2: random_str(vec_size),
        data3: random_str(vec_size),
        data4: random_str(vec_size),
        data5: random_str(vec_size),
        data6: random_str(vec_size),
        data7: random_str(vec_size),
        data8: random_str(vec_size),
        data9: random_str(vec_size),
    }
}

static INI_STR: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

fn create_random_str(vec_size: usize) -> String {
    let mut s = String::with_capacity(vec_size);
    let mut rng = rand::thread_rng();
    for _ in 0..vec_size {
        s.push(rng.gen_range('!'..='~'));
    }
    s
}

fn random_str(vec_size: usize) -> String {
    let mut s = INI_STR.lock().unwrap();
    if s.len() != vec_size {
        *s = create_random_str(vec_size);
    }
    s.clone()
}
