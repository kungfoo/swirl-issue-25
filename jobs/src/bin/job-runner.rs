#[macro_use]
extern crate log;

use db::connection::init_pool;
use simple_logger::SimpleLogger;
use std::thread::sleep;
use std::time::Duration;
use swirl::Runner;

use jobs::images::scale_image;

fn main() {
    // make sure to 'use' this job method, so it's not unused
    let _jobs = [scale_image];

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
