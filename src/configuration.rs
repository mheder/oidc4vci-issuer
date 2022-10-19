use oidc4vci_rs::SSI;
use rocket::{get, serde::json::Json, State};
use serde_json::{json, Value};

use crate::Config;

fn metadata(config: &State<Config>) -> Json<Value> {
    Json(json!({
       "issuer": config.issuer,
       "credential_endpoint": format!("{}/credential", config.issuer),
       "token_endpoint": format!("{}/token", config.issuer),
       "jwks_uri": format!("{}/jwks", config.issuer),
       "grant_types_supported": [
          "urn:ietf:params:oauth:grant-type:pre-authorized_code"
       ],
       "credentials_supported": {
          "OpenBadgeCredential": {
             "formats": {
                "jwt_vc": {
                   "types": [
        "https://imsglobal.github.io/openbadges-specification/ob_v3p0.html#OpenBadgeCredential",
        "https://w3id.org/ngi/OpenBadgeExtendedCredential"
                   ],
                   "binding_methods_supported": [
                      "did"
                   ],
                   "cryptographic_suites_supported": [
                      "ES256"
                   ]
                }
             }
          }
       }
    }))
}

#[get("/.well-known/openid-configuration")]
pub fn openid_configuration(config: &State<Config>) -> Json<Value> {
    metadata(config)
}

#[get("/.well-known/oauth-authorization-server")]
pub fn oauth_authorization_server(config: &State<Config>) -> Json<Value> {
    metadata(config)
}

#[get("/.well-known/openid-credential-issuer")]
pub fn verifiable_credentials_server(config: &State<Config>) -> Json<Value> {
    metadata(config)
}

#[get("/jwks")]
pub fn jwks(interface: &State<SSI>) -> Json<Value> {
    let jwk = interface.jwk.to_public();

    Json(json!({
        "keys" : vec![jwk],
    }))
}

#[get("/.well-known/did.json")]
pub fn did_web(config: &State<Config>, interface: &State<SSI>) -> Json<Value> {
    let Config { issuer, .. } = config.inner();

    let did_web = format!("did:web:{}", issuer);

    Json(json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": did_web,
        "verificationMethod": [{
            "id": format!("{}#controller", did_web),
            "type": "JsonWebKey2020",
            "controller": did_web,
            "publicKeyJwk": interface.jwk.to_public(),
        }],
        "authentication": [did_web],
        "assertionMethod": [did_web],
    }))
}
