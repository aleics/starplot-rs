# <img src="img/starplot-icon.png" alt="starplot-icon" width="20"/> starplot-rs

[![Build Status](https://travis-ci.org/aleics/starplot-rs.svg?branch=master)](https://travis-ci.org/aleics/starplot-rs) [![License](https://img.shields.io/crates/l/cage.svg)](https://opensource.org/licenses/MIT) 

A tool written in [Rust](https://www.rust-lang.org) for visualization of Star Plots.

## contribute
Everyone is free to contribute. If interested:

1. Fork the repository: https://github.com/aleics/starplot-rs/fork
2. Create your branch: `$ git checkout -b new-branch`
3. Commit changes: `$ git commit -am 'new stuff!'`
4. Push it to your branch: `$ git push origin new-branch`
5. Create a new pull request

## download
Clone the repository as follows:

```
    $ git clone https://github.com/aleics/starplot-rs
```

Or download the repository as a zip.

## build
Enter on the repository directory and build it using `cargo` with the nightly version of Rust:

```
    $ cd starplot-rs
    $ cargo build --verbose
```

Or just use [rustup](https://github.com/rust-lang-nursery/rustup.rs):

```
    $ rustup run nightly cargo build
```

## run
Go to the `target/debug` or `target/release` folder and execute `main`:

```
    $ cd target/debug
    $ ./main
```

After you should see something similiar as:

<img src="img/starplot-1.png" alt="starplot-night" width="500"/>

Press after `Q` or `ESC` to exit.

## reading configuration file
It's also possible to read a JSON configuration (look in `conf/example.json`) using the 
`read_conf` method from the `App` struct. Afterwards, the absolut path of the configuration
file can be passed as an argument when calling the compiled file:

```
    $ ./file /home/user/config.json
```


## commands

* `I`: switch night/day view (invert).
* `Q`: exit the application.
* `ESC`: exit the application. 
* `R`: rotate Star Plot.