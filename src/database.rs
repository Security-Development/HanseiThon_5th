
use mysql::Pool;

pub fn data_base_connection() -> Pool{
    let url = "mysql://root:lee12345@localhost:3392/hansei";
    return Pool::new(url).expect("Query connection failed");
}
