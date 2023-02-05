pub type UserId = String;

pub struct UserInformation {
	pub user_id: UserId,
	pub username: String,
	pub email: Option<String>,
	pub password: String,
	pub admin: bool,
	pub bot: bool,
}

pub struct PlayerInformation {
	pub user_id: UserId,
	pub username: String,
}
