mod my_autofill;

use actix::Arbiter;
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use my_autofill::{runner, Config, Instruction, platforms::{Platform, get_platform_instructions}};

#[derive(serde::Deserialize, serde::Serialize)]
struct AutofillPayload {
    platforms: Vec<Platform>
}

#[post("/")]
async fn test(body: web::Json<AutofillPayload>) -> impl Responder {
    let payload = body.into_inner();
    let mut instructions: Vec<Vec<Instruction>> = Vec::new();
    for platform in payload.platforms {
        let platform_process = get_platform_instructions(platform);
        instructions.push(platform_process);
    }
    let config = Config{ instructions };

    Arbiter::current().spawn(async {
        let response = runner(config).await;
        if let Err(err) = response {
            dbg!(err);
        } 
    });

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started 🚀");
    HttpServer::new(|| App::new().service(test))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await    
}
