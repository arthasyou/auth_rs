use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};
use crate::settings::JwtCfg;
use std::mem::MaybeUninit;
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    fn new(aud: String, sub: String, exp: usize) -> Self {
        Claims {
            aud,
            sub,
            exp,
        }
    }
}

pub enum TokenKind {
    ACCESS,
    REFRESH
}

// ==========================================================================================
pub fn general_expired_time(duration: usize) -> usize {
    let exp = Utc::now() + Duration::seconds(duration as i64);
    exp.timestamp() as usize
}

pub fn general_token(kind: TokenKind, sub: String) -> String {
    unsafe {
        let jwt = JWT_STRUCT.as_ptr();
        let aud = (*jwt).aud.clone();
        let (key, exp) = match kind {
            TokenKind::ACCESS => {
                let duration = (*jwt).duration.clone();
                let exp = general_expired_time(duration);
                (&(*jwt).eak, exp)
            }
            TokenKind::REFRESH => {
                (&(*jwt).erk, 0)
            }
        }; 
        let claims = Claims::new(aud, sub, exp);
        encode(&(*jwt).header, &claims, key).unwrap()
    }    
}

pub fn validate_token(kind: TokenKind, token: &str) -> Result<TokenData<Claims>, Error> {
    unsafe {
        let jwt = JWT_STRUCT.as_ptr();
        let (key, validation) = match kind {
            TokenKind::ACCESS => {
                (&(*jwt).dak, &(*jwt).vak)                
            }
            TokenKind::REFRESH => {
                (&(*jwt).drk, &(*jwt).vrk)
            } 
        };
        decode::<Claims>(token, key, validation)
    } 
}

// ==========================================================================================

#[derive(Clone)]
struct Jwt {
    header: Header,
    eak: EncodingKey,       // encoding_asseec_key    
    erk: EncodingKey,       // encoding_refresh_key
    dak: DecodingKey,       // dencoding_asseec_key
    drk: DecodingKey,       // dencoding_refresh_key
    vak: Validation,        // validation_of_access_token
    vrk: Validation,        // validation_of_refresh_token
    aud: String,
    duration: usize 
}

static mut JWT_STRUCT: MaybeUninit<Jwt> = MaybeUninit::uninit();

pub fn init(cfg: JwtCfg) {
    let header = Header::default();
    let eak = EncodingKey::from_secret(cfg.access_secret.as_bytes());    
    let erk = EncodingKey::from_secret(cfg.refresh_secret.as_bytes());
    let dak = DecodingKey::from_secret(cfg.access_secret.as_bytes());
    let drk = DecodingKey::from_secret(cfg.refresh_secret.as_bytes());    
    let mut vak = Validation::default();
    vak.set_audience(&[cfg.aud.clone()]);
    let mut vrk = vak.clone();
    vrk.validate_exp = false;
    vrk.required_spec_claims.clear();
    let aud = cfg.aud;
    let duration = cfg.duration;

    unsafe {
        JWT_STRUCT.as_mut_ptr()
            .write(Jwt {header, eak, dak, erk, drk, vak, vrk, aud, duration});
    }
}