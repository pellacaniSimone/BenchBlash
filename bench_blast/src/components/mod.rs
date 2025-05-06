pub use ui::templates::*;
pub mod ui {
  pub mod templates;
}


pub use models::sqlite_test_db::*;
pub mod models{
  pub mod sqlite_test_db;
}



pub use views::prime_cpu_bench::*;
pub mod views{
  pub mod prime_cpu_bench;
  pub mod multicore_cpu_benchmark;
}

pub use urls::*;
pub mod urls;



