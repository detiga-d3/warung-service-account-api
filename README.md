# Account Service

Account service for Warung app [API microservices](https://www.notion.so/API-Services-Overview-22c2e1489e188120900bd1be1fe868ea?source=copy_link).

## Requirement

- `rust >= 1.88.0` (Production)
- `docker >= 28.2.2` (Development)

## Development

First thing first, install rust. The easiest way of doing that is by using [rustup](https://www.rust-lang.org/tools/install).

To run the server on development mode, use the command below.

```shell
cargo run
```
 

## Production

List of available environment can be seen at `.env.example`. 

You can start production server using the command below.

```shell
./bin/start
```