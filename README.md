# electron-injector

`electron-injector` is an open source command-line tool written in Rust that enables you to inject JavaScript code into Electron apps at runtime. It is inspired by the Python project [electron-inject](https://github.com/tintinweb/electron-inject/) and offers similar functionality.

## Getting Started

### Installation

You can download pre-compiled binaries from the [releases](https://github.com/itsKaynine/electron-injector/releases) page and add the binary to your `PATH` environment variable.

Alternatively, you can install electron-injector using Cargo, Rust's package manager.

```bash
$ cargo install electron-injector
```

### Usage

To use electron-injector, simply specify the path to the Electron app and the JavaScript file that you want to inject.

```bash
$ electron-injector --script=/path/to/script.js /path/to/electron/app
```

### Options

```
-a, --arg <ARG>          Additional arg for the electron app
-s, --script <SCRIPT>    Path to the javascript file to be injected    
    --host <HOST>        The remote debugging host [default: 127.0.0.1]
-p, --port <PORT>        The remote debugging port [default: 8315]     
-t, --timeout <TIMEOUT>  Timeout in ms for injecting scripts [default: 10000]
-d, --delay <DELAY>      Delay in ms to wait after spawning the process [default: 10000]
    --prelude            Enable prelude script
-h, --help               Print help
-V, --version            Print version
```

## Contributing

We welcome contributions from the community. To contribute to `electron-injector`, please follow these steps:

### Fork the repository

1. Create a new branch for your changes
2. Make your changes and commit them
3. Push your changes to your forked repository
4. Submit a pull request
5. Please ensure that your code adheres to the Rust [coding style guidelines](https://www.rust-lang.org/policies/code-of-conduct#coding-style) and is properly formatted using [rustfmt](https://github.com/rust-lang/rustfmt).

## License

electron-injector is dual licensed under the [MIT License](https://opensource.org/licenses/MIT) and [Apache-2.0 License](https://opensource.org/licenses/Apache-2.0). See the LICENSE-MIT and LICENSE-APACHE-2.0 files for more information.
