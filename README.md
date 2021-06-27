# HTTP Multithread in Rust

Some tests to learn rust.

## Docs

`cargo doc`

Access `target/doc`

## Run

`cargo build`

`cargo run <port>`

## Example's GET

`curl -i -H "Accept: text/html" -H "Content-Type: text/html" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: text/plain" -H "Content-Type: text/plain" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: text/css" -H "Content-Type: text/css" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: text/javascript" -H "Content-Type: text/javascript" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: application/json" -H "Content-Type: application/json" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: text/markdown" -H "Content-Type: text/markdown" -X GET http://127.0.0.1:8000`

`curl -i -H "Accept: image/jpeg" -H "Content-Type: image/jpeg" -X GET http://127.0.0.1:8000`
