use crate::app::scenes::scene::Scene;

pub enum Message {
	ChangeScene(Box<dyn Scene>),
}

