# Mnemonic Worker

This worker generates mnemonic phrases. It uses the rust [mnemonic crate](https://crates.io/crates/mnemonic) and [getrandom](https://crates.io/crates/getrandom)

# Routes

## GET /new

Returns an array of 24 words. For example:

```
GET /new

[
    "uniform",
    "equal",
    "olivia",
    "rhino",
    "orchid",
    "herbert",
    "toyota",
    "flower",
    "quality",
    "table",
    "plume",
    "dublin",
    "bali",
    "store",
    "dominic",
    "lotus",
    "camel",
    "protein",
    "carrot",
    "plaza",
    "speed",
    "remark",
    "pyramid",
    "john"
]
```