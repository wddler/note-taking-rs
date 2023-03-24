// The code below is an adapted and extended version of the code from Josh Amata's article 
// "Building REST APIs in Rust with Actix Web", 2022, https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web

use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::{StatusCode, Method};
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
  id: u32,
  text: String,
}

#[derive(Debug, Serialize)]
struct ErrNoId {
  id: u32,
  err: String,
}

struct AppState {
    notes: Mutex<Vec<Note>>,
}

// Implement Responder Trait for Note
impl Responder for Note {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}

// Implement ResponseError for ErrNoId
impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
 
    fn error_response(&self) -> HttpResponse<BoxBody> {
       let body = serde_json::to_string(&self).unwrap();
       let res = HttpResponse::new(self.status_code());
       res.set_body(BoxBody::new(body))
    }
 }

// Implement Display for ErrNoId
impl Display for ErrNoId {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
   }
} 

async fn health_check(req: HttpRequest) -> HttpResponse {
    // only GET request is expected because I want "sad path" to be tested using native actix_web unit tests.
    match req.method() {
        &Method::GET => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().finish()
    }
}

// Get all notes
#[get("/notes")]
async fn get_notes(data: web::Data<AppState>) -> impl Responder {
   let notes = data.notes.lock().unwrap();

   let response = serde_json::to_string(&(*notes)).unwrap();

   HttpResponse::Ok()
       .content_type(ContentType::json())
       .body(response)
}

// async fn index() -> impl Responder {
//     "Hello world!"
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
                        notes: Mutex::new(vec![
                            Note {
                                id: 1,
                                text: String::from("actix-web seems to be a handy back-end framework")
                            },
                            Note {
                                id: 2,
                                text: String::from("Web application is supposed to be properly tested")
                            }
                        ])
                    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_notes)
            .route("/health_check", web::get().to(health_check))
            // .route("/index.html", web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType, Method},
        test,
    };

    // I use native Actix_web test tools here due.

    // An example of Actix_web unit test.
    // It can be use with handlers that return HttpResponse and so should be registered manually in the app.
    // I.e. a handler that uses a routing macro (#[get("/notes")]) can be unit tested because it returns a Responder trait.
    // A handler with a routing macro is a subject of a Actix_web integration tests only.
    // Side note: Unit tests for handlers can be omitted in development because they should only 
    // receive a request, call business logic, and prepare a response. And business logic should be unit tested.
    // Although you might want to test a specific response from the handler...

    #[actix_web::test]
    async fn test_health_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = health_check(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_health_not_ok() {
        let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .method(Method::POST)
        .to_http_request();
        let resp = health_check(req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    // actix_web integration tests
    #[actix_web::test]
    async fn test_get_notes_ok() {
        let test_notes = vec![
            Note {
                id: 1,
                text: String::from("actix-web seems to be a handy back-end framework")
            },
            Note {
                id: 2,
                text: String::from("Web application is supposed to be properly tested")
            }
        ];
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    notes: Mutex::new(test_notes.clone())
                }))
                .service(get_notes)).await;
        let req = test::TestRequest::get().uri("/notes").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let resp_body = test::read_body(resp).await;
        let notes: Vec<Note> = serde_json::from_slice(&resp_body).unwrap();
        assert_eq!(test_notes.len(), notes.len());
    }
    
    #[actix_web::test]
    async fn test_get_notes_not_ok() {
        let test_notes = vec![
            Note {
                id: 1,
                text: String::from("actix-web seems to be a handy back-end framework")
            },
            Note {
                id: 2,
                text: String::from("Web application is supposed to be properly tested")
            }
        ];
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    notes: Mutex::new(test_notes.clone())
                }))
                .service(get_notes)).await;
        let req = test::TestRequest::post().uri("/notes").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }


}
