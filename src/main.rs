#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize)]
struct MortgageInput {
    principal: f64,
    annual_interest_rate: f64,
    years: i32,
}

#[derive(Serialize)]
struct MortgageOutput {
    monthly_payment: f64,
}

#[post("/calculate", format = "json", data = "<input>")]
fn calculate(input: Json<MortgageInput>) -> Json<MortgageOutput> {
    let monthly_interest_rate = input.annual_interest_rate / 100.0 / 12.0;
    let payments = input.years * 12;
    let x = (1.0 + monthly_interest_rate).powi(payments);
    let monthly_payment = input.principal * monthly_interest_rate * x / (x - 1.0);

    Json(MortgageOutput { monthly_payment })
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, calculate])
}
