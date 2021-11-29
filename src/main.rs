use chrono::{Duration, prelude::*};


// 执行命令之后 获取当前周一 - 周五的 fmt string & print

fn main() {
    let local: DateTime<Local> = Local::now();
    let week = local.weekday().number_from_monday() - 1;
    let monday = local - Duration::days(week.into());
    let friday = monday + Duration::days(5 - 1);
    print!("{} - {}", monday.format("%Y%m%d"), friday.format("%Y%m%d"));
}
