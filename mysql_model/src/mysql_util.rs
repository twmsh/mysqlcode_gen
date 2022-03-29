use chrono::prelude::*;


// 从驱动层(sqlx)读取的时间，被认为是UtC时间，实际不是
pub fn fix_read_dt(dt: &mut DateTime<Local>, db_offset: &FixedOffset) {
    let origin_dt = dt.with_timezone(&Utc);
    let db_local = db_offset
        .from_local_datetime(&origin_dt.naive_local())
        .unwrap();

    let dst_dt = db_local.with_timezone(&Local);
    *dt = dst_dt;
}

// 驱动层(sqlx)会将DateTime<Local>类型，转成utc时间。对NaiveDateTime类型不做变动
pub fn fix_write_dt(dt: &DateTime<Local>, db_offset: &FixedOffset) -> NaiveDateTime {
    let db_local = dt.with_timezone(db_offset);
    db_local.naive_local()
}

// 解析字符串得到时区 例如："+08:00"
pub fn parse_timezone(tz: &str) -> std::result::Result<FixedOffset, String> {
    let list: Vec<char> = tz.chars().collect();
    println!("list: {:?}", list);
    if list.len() < 6 {
        return Err(format!("{} is invalid timezone", tz));
    }
    let east;
    match list[0] {
        '+' => east = true,
        '-' => east = false,
        _ => {
            return Err(format!("{} is invalid timezone", tz));
        }
    }

    let ts = match NaiveTime::parse_from_str(&tz[1..], "%H:%M") {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("{} is invalid timezone, {:?}", tz,e));
        }
    };

    let seconds = ts.hour() * 60 * 60 + ts.minute() * 60 + ts.second();
    if east {
        Ok(FixedOffset::east(seconds as i32))
    } else {
        Ok(FixedOffset::west(seconds as i32))
    }
}
