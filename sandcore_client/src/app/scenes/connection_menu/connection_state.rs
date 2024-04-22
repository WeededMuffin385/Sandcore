#[derive(Default)]
pub enum ConnectionState{
	#[default]
	Idle,
	Process,
	Success,
	Failure,
}