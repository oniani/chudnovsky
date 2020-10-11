# Chudnovsky

The Chudnovsky algorithm for calculating the digits of Pi.

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

One could also directly change `ITERATIONS` and `PRECISION` constants in
`src/main.rs`.

The bigger both variables, the more digits are going to be generated.

## References

- [rug](https://docs.rs/rug/1.11.0/rug/index.html)
- [Chudnovsky algorithm](https://en.wikipedia.org/wiki/Chudnovsky_algorithm)

## License

[GNU General Public License v3.0](LICENSE)
