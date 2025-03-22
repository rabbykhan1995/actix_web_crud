use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

pub async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>, // ✅ Fix: Ensure 'static lifetime
) -> Result<ServiceResponse<BoxBody>, Error> {
    // 🔹 Try to get the "token" cookie
    let cookie = req.cookie("token");

    // ❌ If cookie is missing, return 401 Unauthorized
    if cookie.is_none() {
        println!("❌ No cookie found!");
        return Ok(req.into_response(HttpResponse::Unauthorized().body("No cookie found")));
    }

    // ✅ Cookie exists, extract token value
    let token = cookie.unwrap().value().to_string();
    println!("✅ Found Token: {}", token);

    // ❌ Reject empty tokens
    if token.is_empty() {
        println!("❌ Empty Token!");
        return Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid Token")));
    }

    // 🔹 Validate the token (dummy example)
    if !validate_token(&token) {
        println!("❌ Invalid Token!");
        return Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid Token")));
    }

    // ✅ Token is valid, proceed to next middleware/handler
    next.call(req).await.map(|res| res.map_into_boxed_body())
}

// 🔹 Example Token Validation (Replace with real JWT validation)
fn validate_token(token: &str) -> bool {
    token == "valid_token_123" // Dummy validation
}
