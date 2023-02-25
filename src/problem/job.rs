pub struct Job{
    pub id: String,
    pub location: (f64, f64),
    pub demand: f64,
    pub service: f64,
    pub time_windows: (f64, f64),
    pub priority: i32,
}