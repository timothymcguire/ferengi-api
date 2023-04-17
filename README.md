# ferengi-api
A simple API built in Rust using Actix for retrieving Star Trek's Ferengi Rules of Acquisition. This is still a WIP, but is in a functional state.

## Usage
Simply download the repo, and run ```cargo run``` in the root directory to run the server.

### APIs
#### `GET /rule`
Returns one random rule of acquisition in an array.
```json
[ 
    { 
        "num": 1,
        "text": "Once you have their money, you never give it back." 
    } 
]
```

### `GET /rule/{rule_num}`
Returns one rule of acquisition in an array by the rule's number.
```json
[
    { 
        "num": 1,
        "text": "Once you have their money, you never give it back." 
    }
]

## Acknowledgements
 - The rules were sourced from the [Memory Alpha](https://memory-alpha.fandom.com/wiki/Rules_of_Acquisition) wiki.
