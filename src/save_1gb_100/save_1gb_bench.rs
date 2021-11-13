use crate::save_1gb_100::save_1gb_accessor::RootIntf;
use crate::util::get_file_lens;
use dochy::error::DpResult;
use dochy::fs::common::CurrentSrc;
use dochy::fs::history::{list_histories, save_history_file, HistoryInfo};
use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Instant;

pub const VEC_SIZE: usize = 1_000_000;
pub const LOOP: usize = 100;

pub(crate) fn save_1gb_bench() -> DpResult<()> {
    let history_dir = "src/save_1gb_100/history_dir";
    std::fs::remove_dir_all(history_dir).ok();
    std::fs::create_dir(history_dir).ok();

    let src_dir = "src/save_1gb_100/src_dir";
    let info = HistoryInfo::create(history_dir, CurrentSrc::from_src_dir(src_dir), ())?;

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    init(&mut root);
    let start_time = Instant::now();
    for _ in 0..LOOP {
        modify(&mut root);
        save_history_file(&info, None, root.root_obj_ref())?;
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} milliseconds", duration.as_millis());

    let his = list_histories(&info)?;
    let hash_dir = his
        .get_newest_file_data().unwrap()
        .calc_path(history_dir)
        .parent().unwrap()
        .to_path_buf();
    let lens = get_file_lens(&hash_dir, "his")?;
    let sum_files = lens.iter().fold(0, |sum, fl| sum + fl.len());
    println!("sum of file sizes {}", sum_files);
    for fl in &lens {
        //println!("file {} len {}", fl.file_name(), fl.len())
    }
    Ok(())
}

fn modify(root: &mut RootIntf) {
    let mut rng = rand::thread_rng();
    let r: usize = rng.gen_range(0..10);
    match r {
        0 => modify_vec(root.data0_mut()),
        1 => modify_vec(root.data1_mut()),
        2 => modify_vec(root.data2_mut()),
        3 => modify_vec(root.data3_mut()),
        4 => modify_vec(root.data4_mut()),
        5 => modify_vec(root.data5_mut()),
        6 => modify_vec(root.data6_mut()),
        7 => modify_vec(root.data7_mut()),
        8 => modify_vec(root.data8_mut()),
        9 => modify_vec(root.data9_mut()),
        _ => unreachable!(),
    }
}

fn modify_vec(vec: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..vec.len());
    vec[i] = gen_ascii(&mut rng);
}

fn init(root: &mut RootIntf) {
    root.set_data0(initial_vec());
    root.set_data1(initial_vec());
    root.set_data2(initial_vec());
    root.set_data3(initial_vec());
    root.set_data4(initial_vec());
    root.set_data5(initial_vec());
    root.set_data6(initial_vec());
    root.set_data7(initial_vec());
    root.set_data8(initial_vec());
    root.set_data9(initial_vec());
}

static INI_VEC: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut v = Vec::with_capacity(VEC_SIZE);
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_SIZE {
        v.push(gen_ascii(&mut rng));
    }
    v
});

fn initial_vec() -> Vec<u8> {
    INI_VEC.clone()
}

pub(crate) fn gen_ascii(rng: &mut ThreadRng) -> u8 {
    rng.gen_range(('!' as u8)..=('~' as u8))
}
