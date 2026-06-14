# Skillful Hands Solutions LTD — website

Маркетинговый сайт на **Rust + Dioxus (web / WASM)**. Без бэкенда и авторизации —
статический лендинг, который билдится в `dist/` и кладётся на любой static-хостинг
(GitHub Pages и т.п.).

## Стек
- Rust + Dioxus 0.7 (web)
- Сборка через `dx` (Dioxus CLI)

## Запуск (dev)
```bash
dx serve
```
Откроется на `http://127.0.0.1:8080`.

## Прод-сборка
```bash
dx build --release
```
Готовый сайт — в `dist/`.

## Структура
```
Skillful_hands/
├── Cargo.toml          # зависимости (dioxus web)
├── Dioxus.toml         # конфиг dx (title, public_dir)
├── index.html          # шапка: шрифты, meta, точка монтирования #main
├── assets/
│   ├── main.css        # base + брендовые токены (золото/тёмный)
│   └── img/            # логотип
└── src/
    ├── main.rs         # точка входа (dioxus::launch)
    ├── app.rs          # корневой компонент App, сборка секций
    └── components/     # секции лендинга (по дизайну из Pencil)
```

## Дизайн
Источник истины — Pencil-файл `alexander_site500.pen`. Секции верстаются node-by-node
по макету. Брендовые цвета/шрифты в `assets/main.css` синхронизируются с переменными Pencil.
