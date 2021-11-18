// use super::schema::posts;

#[derive(Queryable)]
pub struct Service {
	pub id: i32,
	pub secret: String
}
// TODO: add features for insertion
// #[derive(Insertable)]
// pub struct Service {
// 	pub id: i32,
// 	pub secret: String
// }