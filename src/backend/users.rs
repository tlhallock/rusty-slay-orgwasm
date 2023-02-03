pub type UserId = String;

pub struct UserInformation {
	user_id: UserId,
	username: String,
	email: Option<String>,
	password: String,
	admin: bool,
	bot: bool,
}

pub struct PlayerInformation {
	user_id: UserId,
	username: String,
}
