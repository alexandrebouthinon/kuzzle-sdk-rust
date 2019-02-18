<p align="center">
  <h1>Contributing to the Kuzzle Rust SDK</h1>
</p>

<p align="center">
  <b>:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:</b>
</p>

## General
If you plan to add new features to Kuzzle Rust SDK, make sure that this is for a general improvement to benefit a majority of Kuzzle users. 
> You want to discuss about an idea, a problem? Come join the team on [Gitter](https://gitter.im/kuzzleio/kuzzle).

In any case, here are some rules that we would like to follow throughout the SDK development:
* [Follow the KISS Principle](https://en.wikipedia.org/wiki/KISS_principle)
* [Follow The Boy Scout Rule](http://programmer.97things.oreilly.com/wiki/index.php/The_Boy_Scout_Rule)

## Environment

The SDK is built using the Rust `stable` toolchain and `2018 edition` syntax.
If you want to contribute, first of all ensure your environnement is ready and up to date.

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

## Guidelines

In order for the code to maintain a certain consistency and acceptable test coverage,
please follow these guidelines:

* __Never use unsafe or nightly Rust features.__
* Each function made available through the library's public API must:
    * be tested through a series of unit tests.
    * be briefly documented and include a simple example of use.
* Same for fairly complex private functions.
> Feel free to use inline comments to explain your contributions, which you may find a little too complex.
* Each pull request branch must be based on `1-dev` and be up to date.
* Prefer rebase over merge for updating your branch.
* Avoid commit messages such as `WIP` or `Fix stuff` to make code review easier.
* Avoid pushing each commit one by one to avoid saturating Travis builds.
