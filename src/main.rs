use actix_web::{App, HttpResponse, HttpServer, Responder, Result, get, post, web};
use chrono::{DateTime, Utc};
use serde::Serialize;

const DEFAULT_PROCESSOR_URL: &str = "http://localhost:8000";
const FALLBACK_PROCESSOR_URL: &str = "http://localhost:8001";
const PAYMENTS_URL: &str = "/payments";

#[derive(Serialize)]
struct SummaryResponse {
    default: Payment,
    fallback: Payment,
}

#[derive(Serialize)]
struct Payment {
    totalRequests: u32,
    totalAmount: f64,
}

#[derive(Serialize, Debug)]
struct NewPayment {
    correlationId: String,
    amount: f64,
    requestedAt: DateTime<Utc>,
}

#[post("/payments")]
async fn payments() -> impl Responder {
    let payload = NewPayment {
        correlationId: "".to_string(),
        amount: 10.0,
        requestedAt: Utc::now(),
    };

    let client = reqwest::Client::new();
    match client
        .post("http://localhost:8001/payments")
        .header("X-Rinha-Token", "2k25")
        .json(&payload)
        .send()
        .await
    {
        Ok(_response) => {
            println!("Response ok");
            HttpResponse::Created()
        }
        Err(_e) => {
            println!("Response failed");
            HttpResponse::InternalServerError()
        }
    };

    HttpResponse::Ok()
}

#[get("/payments-summary")]
async fn payments_summary() -> Result<impl Responder> {
    let resp = SummaryResponse {
        default: Payment {
            totalRequests: 100,
            totalAmount: 200.0,
        },
        fallback: Payment {
            totalRequests: 100,
            totalAmount: 400.0,
        },
    };
    Ok(web::Json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on 9999");
    HttpServer::new(|| App::new().service(payments).service(payments_summary))
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}
