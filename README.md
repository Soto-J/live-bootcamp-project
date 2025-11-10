## Setup & Building

```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)

#### App service

```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service

```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

### Make the script executable: (chmod +x docker.sh)

```bash
docker compose build
docker compose up
```

## Run servers locally (Docker)

```bash
./docker.sh
```

### Start a MySQL instance via Docker
```bash
docker pull mysql:8.0
docker run --name mysql-db -e MYSQL_ROOT_PASSWORD=[MYSQL_ROOT_PASSWORD] -p 3306:3306 -d mysql:8.0



```
visit http://localhost:8000 and http://localhost:3000



