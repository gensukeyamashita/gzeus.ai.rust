use actix_web::HttpResponse;

pub struct HttpRequestHandler;

impl HttpRequestHandler {
    pub fn handle_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Your request handling logic here
        Ok("response".to_string())
    }

    pub fn log_output(&self, response: &str) {
        // Your logging logic here
        println!("Response: {}", response);
    }

    pub fn handle_error(&self, error: &dyn std::error::Error) {
        // Your error handling logic here
        eprintln!("Error: {}", error);
    }

    pub async fn process_request(&self, request: &str) -> HttpResponse {
        match self.handle_request(request) {
            Ok(response) => {
                self.log_output(&response);
                HttpResponse::Ok().body(response)
            }
            Err(e) => {
                self.handle_error(&*e);
                HttpResponse::InternalServerError().body("Failed to make request")
            }

        }
    }
}
