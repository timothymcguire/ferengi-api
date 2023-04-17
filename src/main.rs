use actix_web::{get, web, App, HttpServer, Responder};
use phf::phf_map;
use rand::seq::IteratorRandom;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
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
    Empty,
}

static OFFICIAL_RULES: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "Once you have their money, you never give it back.",
    3u32 => "Never spend more for an acquisition than you have to.",
    6u32 => "Never allow family to stand in the way of opportunity.",
    7u32 => "Keep your ears open.",
    9u32 => "Opportunity plus instinct equals profit.",
    10u32 => "Greed is eternal.",
    16u32 => "A deal is a deal.",
    17u32 => "A contract is a contract is a contract... but only with a Ferengi.",
    18u32 => "A Ferengi without profit is no Ferengi at all.",
    21u32 => "Never place friendship above profit.",
    22u32 => "A wise man can hear profit in the wind.",
    23u32 => "Nothing is more important than your health... except for your money.",
    31u32 => "Never make fun of a Ferengi's mother.",
    33u32 => "It never hurts to suck up to the boss.",
    34u32 => "War is good for business.",
    35u32 => "Peace is good for business.",
    45u32 => "Expand or die.",
    47u32 => "Don't trust a man wearing a better suit than your own.",
    48u32 => "The bigger the smile, the sharper the knife.",
    57u32 => "Good customers are as rare as latinum. Treasure them.",
    59u32 => "Free advice is seldom cheap.",
    62u32 => "The riskier the road, the greater the profit.",
    74u32 => "Knowledge equals profit.",
    75u32 => "Home is where the heart is, but the stars are made of latinum.",
    76u32 => "Every once in a while, declare peace. It confuses the hell out of your enemies.",
    94u32 => "Females and finances don't mix.",
    95u32 => "Expand or die.",
    98u32 => "Every man has his price.",
    102u32 => "Nature decays, but latinum lasts forever.",
    103u32 => "Sleep can interfere with...",
    109u32 => "Dignity and an empty sack is worth the sack.",
    111u32 => "Treat people in your debt like family... exploit them.",
    112u32 => "Never have sex with the boss' sister.",
    125u32 => "You can't make a deal if you're dead.",
    139u32 => "Wives serve, brothers inherit.",
    190u32 => "Hear all, trust nothing.",
    194u32 => "It's always good business to know about new customers before they walk in your door.",
    203u32 => "New customers are like razor-toothed gree-worms. They can be succulent, but sometimes they bite back.",
    208u32 => "Sometimes the only thing more dangerous than a question is an answer.",
    211u32 => "Employees are the rungs on the ladder of success. Don't hesitate to step on them.",
    214u32 => "Never begin a business negotiation on an empty stomach.",
    217u32 => "You can't free a fish from water.",
    223u32 => "Unknown, but presumably concerned the relationship between \"keeping busy\" and \"being successful\".",
    229u32 => "Latinum lasts longer than lust.",
    239u32 => "Never be afraid to mislabel a product.",
    263u32 => "Never allow doubt to tarnish your lust for latinum.",
    285u32 => "No good deed ever goes unpunished.",
    // One more unnumbered rule of acquisition
};

#[get("/rule/{rule_num}")]
async fn rule_by_number(rule_num: web::Path<u32>) -> impl Responder {
    let rule_num = rule_num.into_inner();

    if let Some(rule) = OFFICIAL_RULES.get(&rule_num) {
        web::Json(
            RulesResponse::Rules(
                vec![Rule::new(rule_num, rule.to_string())]
            )
        )
    } else {
        web::Json(RulesResponse::Empty)
    }
}

#[get("/rule")]
async fn random_rule() -> impl Responder {
    let mut rng = rand::thread_rng();
    let rule_num: u32 = *OFFICIAL_RULES.keys().choose(&mut rng).unwrap();

    if let Some(rule) = OFFICIAL_RULES.get(&rule_num) {
        web::Json(
            RulesResponse::Rules(
                vec![Rule::new(rule_num, rule.to_string())]
            )
        )
    } else {
        web::Json(RulesResponse::Empty)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(rule_by_number)
            .service(random_rule)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}