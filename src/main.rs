use mysql::*;
use mysql::prelude::Queryable;

fn main() -> Result<()> {
    // 设置数据库连接选项
    let url = "mysql://root:root@127.0.0.1:3306";
    let opts = Opts::from_url(url)?;
    let pool = Pool::new(opts)?;

    // 获取数据库连接
    let mut conn = pool.get_conn()?;

    // 执行查询并打印所有数据库名称
    let result = conn.query_iter("SHOW DATABASES")?;

    for row in result {
        let db_name: String = row?.get(0).unwrap();
        println!("{}", db_name);
    }

    Ok(())
}
