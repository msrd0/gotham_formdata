use gotham::{
	handler::HandlerError,
	helpers::http::response::*,
	hyper::{Body, Response, StatusCode},
	router::builder::*,
	state::State,
	test::TestServer
};
use gotham_formdata::FormData;
use mime::{Mime, TEXT_PLAIN};
use validator::Validate;

#[derive(FormData, Validate)]
struct LoginData {
	username: String,
	password: String
}

async fn login_handler(state: &mut State) -> Result<Response<Body>, HandlerError> {
	let login_data: LoginData = FormData::parse_form_data(state).await?;
	Ok(if login_data.password == "secret" {
		create_response(state, StatusCode::OK, TEXT_PLAIN, login_data.username)
	} else {
		create_empty_response(state, StatusCode::FORBIDDEN)
	})
}

#[test]
fn urlencoded() {
	let _ = pretty_env_logger::try_init_timed();

	let server = TestServer::new(build_simple_router(|router| {
		router.post("/login").to_async_borrowing(login_handler);
	}))
	.unwrap();

	let mime: Mime = "application/x-www-form-urlencoded".parse().unwrap();
	let body = "username=testuser&password=secret";
	let res = server
		.client()
		.post("http://localhost/login", body, mime)
		.perform()
		.unwrap();
	let body = res.read_body().unwrap();
	assert_eq!(&body, b"testuser");
}
