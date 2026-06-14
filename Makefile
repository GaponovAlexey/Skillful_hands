# Skillful Hands Solutions LTD — dev / прод команды

# Разработка — Dioxus dev сервер с hot-reload на порту 5001
dev:
	dx serve --port 5001

# Билд для production → dist/
build:
	dx build --release

# Алиас: make c
c: dev
