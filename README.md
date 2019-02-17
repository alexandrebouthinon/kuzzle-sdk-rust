<p align="center">
  <img src="https://user-images.githubusercontent.com/7868838/52912711-4c2d0900-32b5-11e9-9064-472b025c886e.png"/>
</p>
<p align="center">
  <img src="https://img.shields.io/badge/tested%20on-linux%20%7C%20osx-blue.svg">
  <a href="https://github.com/alexandrebouthinon/kuzzle_sdk/blob/master/LICENSE">
    <img alt="undefined" src="https://img.shields.io/github/license/alexandrebouthinon/kuzzle_sdk.svg?style=flat">
  </a>
  <a href="https://travis-ci.com/alexandrebouthinon/kuzzle_sdk">
    <img src="https://travis-ci.com/alexandrebouthinon/kuzzle_sdk.svg?branch=1-dev"/>
  </a>
  <a href="https://codecov.io/gh/alexandrebouthinon/kuzzle_sdk">
    <img src="https://codecov.io/gh/alexandrebouthinon/kuzzle_sdk/branch/1-dev/graph/badge.svg" />
  </a>
</p>


## About 

<p align="center">
  <em> :warning: Work in progress. Do not use in production. :warning: </em>
</p>

### Rust SDK

This is the community supported Rust SDK for the free and open-source backend Kuzzle.
The aim of this project is to provide the Rust community with a production-ready 
Kuzzle SDK using only the Rust stable toolchain.

### Kuzzle

Kuzzle is an open-source backend that includes a scalable server, a multiprotocol API,
an administration console and a set of plugins that provide advanced functionalities like real-time pub/sub, 
blazing fast search and geofencing.

* :octocat: __[Github](https://github.com/kuzzleio/kuzzle)__
* :earth_africa: __[Website](https://kuzzle.io)__
* :books: __[Documentation](https://docs-v2.kuzzle.io)__
* :email: __[Gitter](https://gitter.im/kuzzleio/kuzzle)__

## Development

<p align="center">
  <a href="https://waffle.io/alexandrebouthinon/kuzzle_sdk">
    <img src="https://badge.waffle.io/alexandrebouthinon/kuzzle_sdk.svg?columns=all" alt="Waffle.io - Columns and their card count">
  </a>
</p>

The SDK is built using the Rust `stable` toolchain and `2018 edition` syntax.
If you want to contribute, first of all: thank you! :tada:; 
end before continue, ensure your development environnement is ready and up to date.

### Rust toolchain

Install latest stable Rust version using `rustup`:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

or update your Rust installation:

```bash
$ rustup update
```

and before continue set your default toolchain to `stable`:

```bash
$ rustup default stable
```

### RustFmt

To avoid about code format unwanted divergence, please install `rustfmt` for your toolchain:

```bash
$ rustup component add rustfmt
```

And run following command at project root before commiting:

```bash
$ cargo fmt
```

### Clippy

Clippy is not required but very useful to find non-idiomatic Rust code.
You can install it using `rustup`:

```bash
$ rustup update
```

```bash
$ rustup component add clippy
```

Take a look to its [documentation](https://github.com/rust-lang/rust-clippy).

### Tests

Ensure your work passe all the tests. You'll need `docker` and `docker-compose`
to run a Kuzzle testing stack. There is a script in `.ci` folder:

```bash
$ ./.ci/start_kuzzle.sh
```

Wait for its completion then run:

```bash
$ cargo test --all
```

Will be run:
* Unit tests
* Documentation tests
* Functional tests (using [cucumber-rust](https://github.com/bbqsrc/cucumber-rust))

### Guidelines

In order for the code to maintain a certain consistency and acceptable test coverage,
please follow these guidelines:

* __Never use unsafe features or nightly features.__
* Each function made available through the library's public API must:
    * be tested through a series of unit tests.
    * be briefly documented and include a simple example of use.
    * Feel free to use inline comments to explain your contributions, which you may find a little too complex.
* Same for fairly complex private functions.
* Each pull request branch must be based on `1-dev` and be up to date.
* Prefer the rebase over the merge for updating your branch.
* Avoid commit messages such as `WIP` or `Fix stuff` to make code review easier.
* Avoid pushing each commit one by one to avoid saturating Travis builds.

<p align="center">
  <a href="https://waffle.io/alexandrebouthinon/kuzzle_sdk/metrics/throughput">
    <img src="https://graphs.waffle.io/alexandrebouthinon/kuzzle_sdk/throughput.svg" alt="Throughput Graph">
  </a>
</p>

