use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, RequestExt, Response};
//use serde::{Deserialize, Serialize};
//use serde_json::json;

async fn handler(event: Request) -> Result<Response<Body>, Error> {
  println!("handler");
  let name = event
    .query_string_parameters_ref()
    .and_then(|params| params.first("name"))
    .unwrap_or("no_name");
  println!("name: {:?}", name);
  
  let id = event.lambda_context().request_id;
  println!("id: {:?}", id);
  let msg = format!("Hello {name}, your requestID:{id}");
  println!("msg: {:?}", msg);
  
  /*let resp_default = Response::builder()
  .status(StatusCode::NOT_FOUND)
  .body("not found".to_owned()).unwrap();
  */
  let resp = Response::builder()
    .status(StatusCode::OK).header("content-type", "application/text")
    .body(msg.into())
    .map_err(Box::new)
    .unwrap();
  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error>{
  //dotenvy::dotenv().expect("Unable to access .env file");

  //set variables from enviroment variables
  //let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
  //let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

  run(service_fn(handler)).await
}
