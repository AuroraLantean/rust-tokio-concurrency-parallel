/* Tokio is using one single thread for its main event loop, so it is good for Non Blocking IO, which is for reading files, waiting for network data

*/
use log::Level;
use tokio::io::AsyncReadExt;
use tokio::{sync, task, time};

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
async fn return_something(d: u64) -> u32 {
  log::info!("return_something");
  time::sleep(time::Duration::from_secs(d)).await;
  log::info!("end_of_return_something");
  5
}
async fn read_file() {
  log::info!("Reading file README.md");
  let mut f = tokio::fs::File::open("README.md").await.unwrap();
  let mut contents = vec![];
  f.read_to_end(&mut contents).await.unwrap();
  log::info!("Read README.md {} bytes", contents.len());

  //run in parallel threads...like Go routines
  task::spawn_blocking(move || {
    log::info!("start a new thread");
    fib(43);
    log::info!("end a new thread");
  })
  .await
  .unwrap();
}
async fn task_spawn_blocking() {
  log::info!("task_spawn_blocking... like Go routines");
  let a = task::spawn_blocking(|| {
    log::info!("Starting fib(40)...");
    let res = fib(40);
    log::info!("fib(40) = {}", res);
  });
  let b = task::spawn_blocking(|| {
    log::info!("Starting fib(39)...");
    let res = fib(39);
    log::info!("fib(39) = {}", res);
  });
  tokio::join!(a, b).0.unwrap();
}
async fn time_out() -> Result<(), Box<dyn std::error::Error>> {
  log::info!("time_out");
  if let Err(_) = time::timeout(time::Duration::from_secs(2), sleep(5)).await {
    log::info!("Sleep() timed out...");
  };
  Ok(())
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

struct MyStruct {
  field: i32,
}
async fn cross_concurrency() {
  log::info!("cross_concurrency");
  let lock = std::sync::Arc::new(sync::Mutex::new(MyStruct { field: 0 }));
  let lock_a = lock.clone();
  let lock_b = lock.clone();
  let a = tokio::spawn(async move {
    let mut val = lock_a.lock().await;
    val.field = 1;
  });
  let b = tokio::spawn(async move {
    let mut val = lock_b.lock().await;
    val.field = 2;
  });
  tokio::join!(a, b).0.unwrap();

  let val = lock.lock().await;
  println!("value field is: {}", val.field)
}

async fn cross_threads() {
  log::info!("cross_threads");
  let (tx, mut rx) = tokio::sync::mpsc::channel(10);
  tokio::spawn(async move {
    for i in 0..10 {
      tx.send(i).await.unwrap();
    }
  });
  while let Some(value) = rx.recv().await {
    println!("received value: {}", value);
  }
}

#[tokio::main]
async fn main() {
  println!("--------== test_async_tokio");
  simple_logger::init_with_level(Level::Info).unwrap();

  //#[tokio::main] can replace below
  //let runtime = tokio::runtime::Runtime::new().unwrap();
  //let future = fire_and_wait();
  //let future = run_in_serial();

  let start_time = std::time::Instant::now();
  //runtime.block_on(future);//replaced by tokio::main
  //fire_and_wait().await;
  //fire_and_forget().await;
  //cross_concurrency().await;
  //cross_threads().await;
  //task_spawn_blocking().await;
  let _ = time_out().await;

  let end_time = std::time::Instant::now();
  log::info!("took {:?} seconds", end_time - start_time);
}

//cargo test -q
#[cfg(test)]
mod tests {
  use super::*;
  //use crate::return_something;

  #[tokio::test]
  async fn test_do_something() {
    let res = return_something(2).await;
    assert_eq!(res, 5);
  }
}
