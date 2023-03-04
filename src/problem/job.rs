#[derive(Default, Debug)]
pub struct Job{
    pub id: String,
    pub location: (f64, f64),
    pub demand: f64,
    pub service_time: f64,
    pub time_windows: (f64, f64),
    pub priority: i32,
}

#[derive(Default, Debug)]
pub struct PDJob{
    pub pickup: Job,
    pub delivery: Job,
}

pub fn distance(job1: &Job, job2: &Job) -> f64 {
    let (x1, y1) = job1.location;
    let (x2, y2) = job2.location;
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
