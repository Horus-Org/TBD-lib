use reqwest::blocking::get;
use reqwest::Error;

fn resolve_did(did_url: &str, resolver_url: &str) -> Result<String, Error> {
    let url = format!("{}?did={}", resolver_url, did_url);
    let response = get(&url)?;
    
    match response.status().is_success() {
        true => Ok(response.text()?),
        false => Err(Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "Failed to resolve DID")),
    }
}

fn main() {
    let did_url = "did:example:123456789abcdefghi";
    let resolver_url = "https://example-did-resolver.com/resolve";

    match resolve_did(did_url, resolver_url) {
        Ok(did_document) => println!("Resolved DID Document: {}", did_document),
        Err(err) => eprintln!("Error: {}", err),
    }
}

