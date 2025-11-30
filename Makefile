.PHONY: all docker-build docker-up docker-down docker-prune migrate

all:
	$(MAKE) docker-down
	$(MAKE) docker-prune
	$(MAKE) docker-build
	$(MAKE) docker-up

docker-build:
	docker compose build

docker-up:
	docker compose up

docker-down:
	docker compose down -v

docker-prune:
	docker system prune -f --volumes

migrate:
	docker compose run migrations
