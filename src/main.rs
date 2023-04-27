mod rules;

use rules::{OFFICIAL_RULES, EXPANDED_RULES};

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use rand::seq::IteratorRandom;
use serde::{Serialize, Deserialize, ser::Serializer};
use serde_json::json;
use phf::Map;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, default_value_t = String::from("0.0.0.0"))]
    hostname: String,

    #[arg(short, long, default_value_t = 80)]
    port: u16
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    num: u32,
    text: String
}

impl Rule {
    pub fn new(num: u32, text: String) -> Self {
        Rule {
            num,
            text
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RulesResponse {
    Rules(Vec<Rule>),
    #[serde(serialize_with="serialize_empty")]
    Empty
}

fn serialize_empty<S>(s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    json!([]).serialize(s)
}

fn rule_by_number(rule_num: u32, table: &Map<u32, &'static str>) -> impl Responder {
    if let Some(rule) = table.get(&rule_num) {
        web::Json(
            RulesResponse::Rules(
                vec![Rule::new(rule_num, rule.to_string())]
            )
        )
    } else {
        web::Json(RulesResponse::Empty)
    }
}

fn random_rule(table: &Map<u32, &'static str>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let rule_num: u32 = *table.keys().choose(&mut rng).unwrap();

    if let Some(rule) = table.get(&rule_num) {
        web::Json(
            RulesResponse::Rules(
                vec![Rule::new(rule_num, rule.to_string())]
            )
        )
    } else {
        web::Json(RulesResponse::Empty)
    }
}

fn random_rules(count: u32, table: &Map<u32, &'static str>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let rules: Vec<Rule> = table
        .entries()
        .map(|x| Rule::new(*x.0, x.1.to_string()))
        .choose_multiple(&mut rng, count as usize);
    web::Json(RulesResponse::Rules(rules))
}

#[get("/rule/number/{rule_num}")]
async fn official_rule_by_number(rule_num: web::Path<u32>) -> impl Responder {
    rule_by_number(rule_num.into_inner(), &OFFICIAL_RULES)
}

#[get("/expanded/number/{rule_num}")]
async fn expanded_rule_by_number(rule_num: web::Path<u32>) -> impl Responder {
    rule_by_number(rule_num.into_inner(), &EXPANDED_RULES)
}

#[get("/rule/{count}")]
async fn official_random_rules(count: web::Path<u32>) -> impl Responder {
    random_rules(count.into_inner(), &OFFICIAL_RULES)
}

#[get("/expanded/{count}")]
async fn expanded_random_rules(count: web::Path<u32>) -> impl Responder {
    random_rules(count.into_inner(), &EXPANDED_RULES)
}

#[get("/rule")]
async fn official_random_rule() -> impl Responder {
    random_rule(&OFFICIAL_RULES)
}

#[get("/expanded")]
async fn expanded_random_rule() -> impl Responder {
    random_rule(&EXPANDED_RULES)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Starting server on port {}...", args.port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(official_rule_by_number)
            .service(expanded_rule_by_number)
            .service(official_random_rule)
            .service(expanded_random_rule)
            .service(official_random_rules)
            .service(expanded_random_rules)
    })
        .bind((args.hostname, args.port))?
        .run()
        .await
}