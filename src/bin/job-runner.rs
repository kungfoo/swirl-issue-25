#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;
use std::thread::sleep;
use std::time::Duration;
use swirl::Runner;
use swirl_issue_25::connection::init_pool;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("starting up...");
    let pool = init_pool();
    let environment = ();

    let runner = Runner::builder(environment).connection_pool(pool).build();
    loop {
        if let Err(e) = runner.run_all_pending_jobs() {
            error!("Something really bad happened when running a job: {}", e);
        }
        sleep(Duration::from_millis(30));
    }
}
