use std::{cmp::Ordering, time::Instant};

use dochy::{
    error::DpResult,
    fs::{
        common::CurrentSrc,
        history::{list_histories, load_history_file, HistoryFileData, HistoryInfo},
    },
};

#[test]
pub(crate) fn load_1gb_bench() -> DpResult<()> {
    let history_dir = "src/save_1gb_100/history_dir";

    let src_dir = "src/save_1gb_100/src_dir";
    let info = HistoryInfo::create(history_dir, CurrentSrc::from_src_dir(src_dir), ())?;

    let his = list_histories(&info)?;
    let mut files = his.list_files();

    files.sort_by(compare);
    let b = files.last().unwrap();
    println!("load {}", b.props().calc_filename());

    let start_time = Instant::now();
    let _hoge = load_history_file(&info, b.props(), b.history(), false)?;
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} milliseconds", duration.as_millis());
    
    Ok(())
}

fn compare(l: &HistoryFileData, r: &HistoryFileData) -> Ordering {
    let l = l.props().order();
    let r = r.props().order();
    let phase_cmp = l.len().cmp(&r.len());
    if phase_cmp != Ordering::Equal {
        return phase_cmp;
    }
    if l.len() == 0 {
        return Ordering::Equal;
    }
    let last_index = l.len() - 1;
    let ord = l[last_index].cmp(&r[last_index]);
    if ord != Ordering::Equal {
        return ord;
    }
    let l_sum : u32 = l.iter().sum();
    let r_sum : u32 = r.iter().sum();
    l_sum.cmp(&r_sum)
}
