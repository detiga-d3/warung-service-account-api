# Account Service

Account service for Warung
app [API microservices](https://www.notion.so/API-Services-Overview-22c2e1489e188120900bd1be1fe868ea?source=copy_link).

## Environment Variables

List of available environment variables can be seen at `.env.example`.

To setup environment quickly on local development, you can copy `.env.example`
into `.env` and edit the value there.

For production, it is not recommended to use `.env` file. It's better to set
them on the server it self.

## Development

To setup local development environment, you need to
install [watchexec](https://github.com/watchexec/watchexec?tab=readme-ov-file#install)
first. It's used to watch changes and reload the server automatically.

You also need rust and docker installed. For rust, I recommend
using [rustup](https://www.rust-lang.org/tools/install) to set them up on your
device.
And for docker,
use [Docker Desktop](https://www.docker.com/products/docker-desktop/).

In the context of development, docker are used to setup application
infrastructure, such as DB.

After everything is setup, you can start the local development server using the
command below.

```shell
./bin/start
```

Or you can just run both command on their own.

```shell
# Setup infrastructure using docker
docker compose up -d

# Start rust server with live reload using watchexec
watchexec -d 1s -e rs,toml -r cargo run
```

## Deploy

Because it is a service in a microservice project, deployment meant to push
docker image into a container registry. To do so, use the command below.

```shell
./bin/deploy
```