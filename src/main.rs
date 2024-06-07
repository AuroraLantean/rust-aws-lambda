use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, RequestExt, Response};
//use serde::{Deserialize, Serialize};
//use serde_json::json;

#[tracing::instrument(skip(event), fields(request_id = %event.lambda_context().request_id))]
async fn handler(event: Request) -> Result<Response<Body>, Error> {
  tracing::info!("handler invoked");//error, warn, info, debug, trace(minimim level to cover all levels)
  let name = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("name"))
    .unwrap_or("no_name");
  tracing::info!("name: {:?}", name);
  
  let id = event.lambda_context().request_id;
  tracing::info!("id: {:?}", id);
  let msg = format!("Hello {name}, your requestID:{id}");
  tracing::info!("msg: {:?}", msg);
  
  /*let resp_default = Response::builder()
  .status(StatusCode::NOT_FOUND)
  .body("not found".to_owned()).unwrap();
  */
  let resp = Response::builder()
    .status(StatusCode::OK).header("content-type", "application/text")
    .body(msg.into())
    .map_err(Box::new)?;
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error>{
  tracing_subscriber::fmt().json().with_max_level(tracing::Level::INFO).with_current_span(false).with_ansi(false).with_target(true).init();
  //.without_time()
  //dotenvy::dotenv().expect("Unable to access .env file");

  //set variables from enviroment variables
  //let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  //let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

  run(service_fn(handler)).await
}
