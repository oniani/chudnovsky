# Chudnovsky

The Chudnovsky algorithm for calculating the digits of Pi.

_Fun fact: it generated over 3 million digits in a little over 7 minutes, and
not on a powerful CPU._

## Dependencies

- [rug](https://docs.rs/rug/1.11.0/rug/index.html)

## Run

```sh
cargo build --release
./target/release/chudnovsky 150 100
```

The output should look like this:

```
Approximation: 3.1415926535897932384626433832795028841971693980
Precision:     150
Iterations:    100
```

The number of iterations and the precision can both be modified via providing
different arguments to the binary. The first and second arguments being the
precision and the number of iterations respectively.

One could also directly change `PRECISION` and `ITERATIONS` constants in
`src/main.rs`.

The bigger both variables, the more digits will be generated.

## Tests

Tests can be run as follows:

```
cargo test --release
```

## References

- [rug](https://docs.rs/rug/1.11.0/rug/index.html)
- [Chudnovsky algorithm](https://en.wikipedia.org/wiki/Chudnovsky_algorithm)

## License

[MIT License](LICENSE)
