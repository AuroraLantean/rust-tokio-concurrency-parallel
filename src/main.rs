/* Tokio is using one single thread for its main event loop, so it is good for Non Blocking IO, which is for reading files, waiting for network data

*/
use log::Level;
use tokio::io::AsyncReadExt;
use tokio::time;

pub fn fib(n: u32) -> u32 {
  match n {
    0 => 0,
    1 => 1,
    n => fib(n - 1) + fib(n - 2),
  }
}
async fn sleep(d: u64) {
  log::info!("Sleeping for {} seconds", d);
  time::sleep(time::Duration::from_secs(d)).await;
  log::info!("Awake from {}", d);
}
async fn do_something(d: u64) {
  log::info!("do_something");
  time::sleep(time::Duration::from_secs(d)).await;
  log::info!("end_of_something");
}
async fn read_file() {
  log::info!("Reading file README.md");
  let mut f = tokio::fs::File::open("README.md").await.unwrap();
  let mut contents = vec![];
  f.read_to_end(&mut contents).await.unwrap();
  log::info!("Read README.md {} bytes", contents.len());

  //run in parallel threads
  tokio::task::spawn_blocking(move || {
    log::info!("start a new thread");
    fib(43);
    log::info!("end a new thread");
  })
  .await
  .unwrap();
}
async fn fire_and_wait() {
  log::info!("fire_and_wait");
  tokio::join!(sleep(1), read_file(), read_file(), read_file(),);
}
async fn run_in_serial() {
  log::info!("run_in_serial");
  for _ in 0..3 {
    read_file().await;
  }
  //sleep(1).await;
}
async fn fire_and_forget() {
  log::info!("fire_and_forget");
  tokio::spawn(async {
    sleep(1).await;
  });
  do_something(1).await;
}

#[tokio::main]
async fn main() {
  println!("--------== test_async_tokio");
  simple_logger::init_with_level(Level::Info).unwrap();

  //#[tokio::main] can replace below
  //let runtime = tokio::runtime::Runtime::new().unwrap();
  //let future = fire_and_wait();
  //let future = run_in_serial2();

  let start_time = std::time::Instant::now();
  //runtime.block_on(future);//replaced by tokio::main
  //fire_and_wait().await;
  fire_and_forget().await;

  let end_time = std::time::Instant::now();
  log::info!("took {:?} seconds", end_time - start_time);
}
