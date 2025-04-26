// needed rusqlite

//#[cfg(feature = "server")]
//thread_local! {
//    pub static DB: rusqlite::Connection = {
//        let conn = rusqlite::Connection::open("benchbalst.db").expect("Failed to open database");
//        conn.execute_batch(
//            "CREATE TABLE IF NOT EXISTS benchmarks (
//                id INTEGER PRIMARY KEY,
//                test TEXT NOT NULL,
//                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
//                result  REAL
//            );",
//        ).unwrap();
//        conn
//    };
//}
//
//#[server]
//pub async fn save_bench(test: String, result:f64) -> Result<(), ServerFnError> {
//    DB.with(|f| f.execute("INSERT INTO benchmarks (test,result) VALUES (?1,?2)", &[&image] , result))?;
//    Ok(())
//}
//
