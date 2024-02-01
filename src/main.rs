use rocket::{ get, http::Status, serde::json::Json };
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status::UiTransactionEncoding;
use solana_sdk::signature::Signature;
use solana_sdk::signature::ParseSignatureError;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct SolanaTransactionResponse {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[get("/txn?<signature>")]
pub async fn get_solana_raw_txn_from_rpc(
    signature: &str
) -> Result<Json<EncodedConfirmedTransactionWithStatusMeta>, Status> {
    const URL: &str = "https://api.mainnet-beta.solana.com";
    let client: RpcClient = RpcClient::new(URL);

     
    println!("Signature: {:?}", signature);
    let bytes: Vec<u8> = bs58
        ::decode(signature)
        .into_vec()
        .map_err(|_| Status::BadRequest)?;
      
    let signature: Signature = match Signature::try_from(bytes.clone()) {
        Ok(sig) => sig,
        Err(_e) => panic!("Expected a length of 64, but got {}", bytes.len()),
    };

    let response = client.get_transaction(&signature, UiTransactionEncoding::JsonParsed).unwrap();
 
    Ok(Json(response))
}

#[get("/health")]
pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Healthy";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

// Change default port for rocket
#[launch]
fn rocket() -> _ {
    rocket
        ::build()
        .configure(rocket::Config {
            port: 8090, // Change this to your desired port
            ..Default::default()
        })
        .mount("/api", routes![health_checker_handler, get_solana_raw_txn_from_rpc])
}
