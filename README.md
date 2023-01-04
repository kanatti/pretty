# Pretty

View files prettified on CLI


## Development

Install as binary:

```sh
$ cargo install --path .
$ pretty --help
Usage: pretty <FILE_NAME>

Arguments:
  <FILE_NAME>

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

Run samples:

```sh
$ pretty samples/test2.json

┌─────────────────┬───────┬──────────────┬──────┬───────┐
│name             │age    │secretIdentity│powers│address│
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Molecule Man"   │29     │"Dan Jukes"   │[..]  │{..}   │
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Madame Uppercut"│39     │"Jane Wilson" │[..]  │{..}   │
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Eternal Flame"  │1000000│"Unknown"     │[..]  │       │
└─────────────────┴───────┴──────────────┴──────┴───────┘

$ pretty samples/test2.json --flatten=address

┌─────────────────┬───────┬──────────────┬──────┬───────────────┬──────────────┬──────────────┐
│name             │age    │secretIdentity│powers│address.country│address.planet│address.galaxy│
├─────────────────┼───────┼──────────────┼──────┼───────────────┼──────────────┼──────────────┤
│"Molecule Man"   │29     │"Dan Jukes"   │[..]  │"US"           │"Earth"       │              │
├─────────────────┼───────┼──────────────┼──────┼───────────────┼──────────────┼──────────────┤
│"Madame Uppercut"│39     │"Jane Wilson" │[..]  │               │"PL120"       │"Andromeda"   │
├─────────────────┼───────┼──────────────┼──────┼───────────────┼──────────────┼──────────────┤
│"Eternal Flame"  │1000000│"Unknown"     │[..]  │               │              │              │
└─────────────────┴───────┴──────────────┴──────┴───────────────┴──────────────┴──────────────┘

```