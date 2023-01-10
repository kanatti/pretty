# Pretty

View files prettified on CLI


## Development

Install as binary:

```sh
$ cargo install --path .
$ pretty --help
Usage: pretty [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -f, --flatten <FIELDS>           Comma seperated list of fields to flatten
  -c, --color <COLOR>              [default: never] [possible values: never, always, auto]
  -s, --select <SELECT>            [default: .]
      --select-mode <SELECT_MODE>  [default: auto] [possible values: only, append, auto]
      --sort <FIELD>               Field to sort by
      --filter <FIELD=VALUE>       Filter expression
  -h, --help                       Print help information
  -V, --version                    Print version information
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

$ pretty samples/test2.json --select 'powers.[].0' --select-mode append

┌─────────────────┬───────┬──────────────┬──────┬───────┬──────────────────────┐
│name             │age    │secretIdentity│powers│address│powers.[].0           │
├─────────────────┼───────┼──────────────┼──────┼───────┼──────────────────────┤
│"Molecule Man"   │29     │"Dan Jukes"   │[..]  │{..}   │"Radiation resistance"│
├─────────────────┼───────┼──────────────┼──────┼───────┼──────────────────────┤
│"Madame Uppercut"│null   │"Jane Wilson" │[..]  │{..}   │"Million tonne punch" │
├─────────────────┼───────┼──────────────┼──────┼───────┼──────────────────────┤
│"Eternal Flame"  │1000000│"Unknown"     │[..]  │       │"Immortality"         │
└─────────────────┴───────┴──────────────┴──────┴───────┴──────────────────────┘

$ pretty samples/test2.json --sort age

┌─────────────────┬───────┬──────────────┬──────┬───────┐
│name             │age    │secretIdentity│powers│address│
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Molecule Man"   │29     │"Dan Jukes"   │[..]  │{..}   │
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Eternal Flame"  │1000000│"Unknown"     │[..]  │       │
├─────────────────┼───────┼──────────────┼──────┼───────┤
│"Madame Uppercut"│null   │"Jane Wilson" │[..]  │{..}   │
└─────────────────┴───────┴──────────────┴──────┴───────┘

# Filter only supports equals for now
$ pretty samples/test2.json --filter 'age=29'

┌──────────────┬───┬──────────────┬──────┬───────┐
│name          │age│secretIdentity│powers│address│
├──────────────┼───┼──────────────┼──────┼───────┤
│"Molecule Man"│29 │"Dan Jukes"   │[..]  │{..}   │
└──────────────┴───┴──────────────┴──────┴───────┘


```

## Enabling color

Use `--color auto` or `--color always` to enable color.

```
$  pretty samples/test2.json --color auto
```

![Colored Ouput](./docs/colored-output.jpg?raw=true "Colored Output")
