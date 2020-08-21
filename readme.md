# epitok-web

Epitok is a library created to replace the token system in the Epitech school.
This is the web client, which supports scanning student cards with QR codes on them to set students present to school events.

This project is written in Rust.

Licensed under [MIT license](license.txt).

## technologies

* [epitok](https://github.com/x4m3/epitok) for the main logic
* [actix-web](https://actix.rs) for the web server
* [askama](https://github.com/djc/askama) for html templates
* [bootstrap](https://getbootstrap.com) for css framework
* [jquery](https://jquery.com) for client side js
* [qr-scanner](https://github.com/nimiq/qr-scanner) for QR code webcam scanning, used at commit [e8a77de](https://github.com/nimiq/qr-scanner/tree/e8a77de)

## development

Tools required: `rust` and `cargo`. You can use [rustup](https://rustup.rs) to install them.

Run `cargo build` to compile and `cargo run` to start the web server.

By default, the server listens on port `4343`. Change this value with the `PORT` environment variable.

⚠️ Warning: The server listens on http only, which means that zero bytes will be encrypted!
There is confidential data that will be transferred between the client and the server, please keep security in mind when deploying.

## deployment

Run `cargo build --release` to compile with optimisations enabled.

The binary will be available at `./target/release/epitok-web`.

Place the binary and the folder `static/` in the same folder, it contains client side css and javascript.