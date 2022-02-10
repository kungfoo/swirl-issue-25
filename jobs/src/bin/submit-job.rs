#[macro_use]
extern crate log;

use db::connection::init_pool;
use jobs::images::{scale_image, Dimension, ScaleImageJob};
use simple_logger::SimpleLogger;
use swirl::Job;
use uuid::Uuid;

fn main() -> Result<(), String> {
    SimpleLogger::new().init().unwrap();
    let pool = init_pool();
    let connection = pool.get().unwrap();
    let id = Uuid::new_v4();

    let src_url = format!("file:///var/swirl-sample/images/{}.jpeg", id);
    let dest_url = format!("file:///var/swirl-sample/images/scaled/{}.jpeg", id);
    let dimension = Dimension {
        width: 1024,
        height: 1280,
    };

    let job = ScaleImageJob {
        src_url,
        dest_url,
        dimension,
    };

    match scale_image(job).enqueue(&connection) {
        Ok(value) => {
            info!("scheduled scale_image job for {}!", id);
            Ok(value)
        }
        Err(e) => {
            error!("Failed to enqueue job due to: {}", e);
            Err(String::from("Failed to enqueue job."))
        }
    }
}
