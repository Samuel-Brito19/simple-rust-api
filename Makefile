DB_DOCKER_CONTAINER=hello_world

install:
# uncomment and indent
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
# SQLX-CLI
	cargo install sqlx-cli

build: 
	cargo build

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 8080:8080 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine