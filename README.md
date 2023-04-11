# contacts-cli
Small app used for learning Rust

## How To Run
```sh
$ make release
$ ./target/release/contacts-cli
```

## Makefile
Check out the makefile for all the available targets
```sh
$ make help

Contacts CLI
Usage: 

  help      Prints this help message
  build     Compile the current package
  update    Update the dependencies of the current package
  run       Run a binary of the local package
  check     Analyze the current package and report errors, but don't build object files
  release   Release the current package
  clean     Clean the current package
  fmt       Format all Rust files of the current crate
  test      Run the tests
```

## Implementation
### Contact
- `phone_no` is validated against `r"49[0-9]{9,10}"`
- `email` is validated against `r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"`
```rust
pub struct Contact {
    pub name: String,
    pub phone_no: u64,
    pub email: String,
}
```
### Collection
In memory tree map, using the `name` as key.
```rust
contacts: BTreeMap<String, Contact>,
```

## Tag Along
- cargo init => [2cfa2b1](https://github.com/MihaiBogdanEugen/contacts-cli/tree/2cfa2b1a89a4e166d16a0d941c4358e74bb99158)
- one makefile to rule them all => [f69fac3](https://github.com/MihaiBogdanEugen/contacts-cli/tree/f69fac32d4b5c97cc11819a43ebd0bc7d9f99363)
- play with btreemap => [92e3036](https://github.com/MihaiBogdanEugen/contacts-cli/tree/92e3036eaf6373574ee0f480dd15a842aaba9fe2)
- validation using regex => [35c2965](https://github.com/MihaiBogdanEugen/contacts-cli/tree/35c2965f25322863ba6a450982085a5f26dc60ec)
- separate business logic => [90e923c](https://github.com/MihaiBogdanEugen/contacts-cli/tree/90e923c55f01c161ba800e8c32fe5c8b6039293d)
- no panics in business logic => [d78bbcb](https://github.com/MihaiBogdanEugen/contacts-cli/tree/d78bbcb7b10dc4f1cd998c4720ce24db950c3c10)
- traits and implementations => [5fc86ff](https://github.com/MihaiBogdanEugen/contacts-cli/tree/5fc86ff94f1a3514befbaaa3f6c8cf3694062031)
- tests => [e233838](https://github.com/MihaiBogdanEugen/contacts-cli/tree/e2338382142bfcaf21ee6480d32c3c427cbb7f1b)
- refined api, separate validation => [06b368a](https://github.com/MihaiBogdanEugen/contacts-cli/tree/06b368adf3ddc5e69c86f9b624f0bb983d9881d0)
- lazy loading for regexes, simplify validation => [6ac683a](https://github.com/MihaiBogdanEugen/contacts-cli/tree/6ac683a9a2ca8d377696c42ccaf28d77308fd2da)
- separate module with private fields and methods => [1bdf4ce](https://github.com/MihaiBogdanEugen/contacts-cli/tree/1bdf4ce891d5971dba0d1e08aef9dc568bb18d3b)
- repl => [574d189](https://github.com/MihaiBogdanEugen/contacts-cli/tree/574d18985fa597a40d9ec9d2f2891a39cd918178)
- export to json => [06f2341](https://github.com/MihaiBogdanEugen/contacts-cli/tree/06f23415868fb56e129fec4420dc07f52f3ef07d)
- import from json => [2489426](https://github.com/MihaiBogdanEugen/contacts-cli/tree/2489426e475e6cc13f9cf71051289a131a224eef)
- refactor error logic => [fcdb8ab](https://github.com/MihaiBogdanEugen/contacts-cli/tree/fcdb8abab2c5674b4ce451f17d23d2be2f558121)
- defensive programming style => [16025e0](https://github.com/MihaiBogdanEugen/contacts-cli/tree/16025e0f3a8cee84b6404cd77594f67ef8ee2f9c)
- list and count => [c56818a](https://github.com/MihaiBogdanEugen/contacts-cli/tree/c56818a4d50723453490ab3f21bb584517fe77df)

## Todos
- search 
