use swirl::{Deserialize, Serialize};

#[swirl::background_job]
pub fn scale_image(job: ScaleImageJob) -> Result<(), swirl::PerformError> {
    info!(
        "scale_image: from: {}, to: {}, dimension: {:?}",
        job.src_url, job.dest_url, job.dimension
    );
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ScaleImageJob {
    pub src_url: String,
    pub dest_url: String,
    pub dimension: Dimension,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}
