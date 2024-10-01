#[derive(Debug, sqlx::FromRow)]
pub struct Unit {
    pub unit_id: String,
    pub rt_temp: String,
    pub rt_pid: String,
    pub blower_trip_fb: bool,
    pub elevator_trip_fb: bool,
    pub rotor_trip_fb: bool,
    pub blower_run_fb: bool,
    pub elevator_run_fb: bool,
    pub rotor_run_fb: bool,
}
