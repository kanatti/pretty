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

```