use chrono::Local;

use mysql_model::cf_model::BeFunction;
use mysql_model::mysql_util;

use mysql_model::mysql_util::MySqxErr;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("aaa");
    let tz = "+08:00";
    let db_url = "mysql://root:cf123456@192.168.1.26:3306/cf_2.6";
    let pool = mysql_util::init_pool(db_url, tz, 2, 1).await?;

    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let entity = BeFunction::load(1, &pool, &offset).await?;
    println!("entity: {:?}", entity);

    let affect = BeFunction::delete(10, &pool).await?;
    println!("affect: {:?}", affect);

    let now = Local::now();

    let mut entity = BeFunction {
        id: 0,
        name: "hahahah".to_string(),
        module_id: 100,
        url: "abcdef".to_string(),
        is_default: 0,
        gmt_create: now,
        gmt_modified: now,
    };

    let affect = entity.insert(&pool, &offset).await?;
    println!("insert: {:?}", affect);

    entity.id = affect as i64;

    let now = Local::now();
    entity.gmt_modified = now;
    entity.module_id = 111;

    let affect = entity.update(&pool, &offset).await?;
    println!("update: {:?}", affect);

    Ok(())
}
