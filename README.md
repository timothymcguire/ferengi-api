# Ferengi Rules of Acquisition API :moneybag:

Introducing "[Ferengi Rules of Acquisition](https://en.wikipedia.org/wiki/Ferengi) API" - the ultimate resource for all 
your [Ferengi](https://en.wikipedia.org/wiki/Ferengi) business needs! Written in Rust using the Actix library, this API 
delivers the 285 Ferengi Rules of Acquisition straight to your fingertips.

Are you tired of constantly referring to a bulky, outdated copy of the Rules of Acquisition? Do you yearn for a more 
efficient way to conduct your intergalactic business dealings? Look no further than our API! With lightning-fast 
response times and rock-solid reliability, you can access the Rules of Acquisition anytime and anywhere programmatically!

Our team of highly skilled Ferengi engineers has painstakingly crafted this API to meet the highest standards of quality 
and performance. We guarantee that you'll be blown away by the level of detail and care that has gone into every aspect 
of our product.

![Shut up and take my latinum!](https://media.tenor.com/EWnaLaRXeKQAAAAC/shut-up-and-take-my-latinum-quark.gif)

But don't just take our word for it - hear what some of our satisfied customers have to say:

"Before I started using the Ferengi Rules of Acquisition API, I was just a simple, naive businessman. But now, thanks 
to the wisdom contained within these rules, I'm a true Ferengi at heart!" - Quark, *Owner of Quark's Bar at Deep Space 9*

"Since I started using this API, my profits have increased by 200%. I can't recommend it enough!" - Brunt, *FCA Liquidator*

So what are you waiting for? Join the ranks of the galaxy's most cunning and successful businesspeople by using 
the Ferengi Rules of Acquisition API today!

## Usage

Simply download the repo, and run ```cargo run``` in the root directory to run the server. The server currently binds
to port 8080.

### APIs

There are two primary APIs that can be interacted with: `/rule` and `/expanded` which return either canon or 
a non-canon rules respectively. The expanded rules include selected rules from non-canon sources such as DS9 novels
to fill in the gaps of the canon rules.

#### Endpoints
|                          Endpoint | Returns                                                      |
|----------------------------------:|--------------------------------------------------------------|
|                       `GET /rule` | an array with one random canon rule                          |
|               `GET /rule/<count>` | an array of random canon rules                               |
|     `GET /rule/number/<rule_num>` | an array with one specific canon rule by its rule number     |
|                   `GET /expanded` | an array with one random non-canon rule                      |
|           `GET /expanded/<count>` | an array of random non-canon rules                           |
| `GET /expanded/number/<rule_num>` | an array with one specific non-canon rule by its rule number |

#### JSON Structure
Rules are represented by JSON objects with two fields, `num` which represents the rule's number, and `text` which is
rule itself.

The API will always return either an array of objects or an empty array if there is nothing.
```json
[
  {
    "num": 1,
    "text": "Once you have their money, you never give it back."
  },
  {
    "num": 9,
    "text": "Opportunity plus instinct equals profit."
  }
]
```

## Disclaimer
Like the original Rules of Acquisition in Star Trek, this API was written for the sake of satire and humor, 
and it is in no way meant to represent the actual beliefs of the author, nor does the author endorse following 
these rules.

As a Star Trek fan, I wrote this as joke and a fun way to experiment with Rust and its libraries, while offering a humorous
API for fellow Star Trek fans. As such, the API and code is bound to change some as I add a few more features such as 
search functionality.

## Acknowledgements
 - The canon rules were sourced from the [Memory Alpha](https://memory-alpha.fandom.com/wiki/Rules_of_Acquisition) wiki.
 - The expanded rules were sourced from the [Memory Beta](https://memory-beta.fandom.com/wiki/Ferengi_Rules_of_Acquisition) wiki