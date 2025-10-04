# ⛅ Weather bot

Telegram-бот для перегляду прогнозу погоди

[![wakatime](https://wakatime.com/badge/user/5f28d705-3bc8-4138-8151-e12e0f9e9a23/project/4d3d812f-19d8-4e42-9ea5-2c7bd2a7e8fb.svg)](https://wakatime.com/badge/user/5f28d705-3bc8-4138-8151-e12e0f9e9a23/project/4d3d812f-19d8-4e42-9ea5-2c7bd2a7e8fb)
![Rust](https://img.shields.io/badge/Rust-1.89.0-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Issues](https://img.shields.io/github/issues/noinsts/weather-rs)
![PRs](https://img.shields.io/github/issues-pr/noinsts/weather-rs)

## ⚡ Коротко

Бот дозволяє реєструвати місто та отримувати швидкий прогноз: сьогодні та завтра. В майбутньому функціонал розширюватиметься.

## 🧭 Основний функціонал

- ✍️ **Реєстрація міста** — кожен користувач зберігає своє місто в SQLite.

- 🌤️ **Прогноз на сьогодні** — температура, погода, вологість, вітер та інші ключові метрики.

- 📅 **Прогноз на завтра** — аналогічно для наступного дня.

- 🔄 **Оновлення прогнозу** — виклик API в будь-який момент.

- 🎨 **Зручний UI** — оформлені повідомлення, callback-кнопки, емодзі.


## 🖼️ Приклад роботи

<img src="assets/weather.jpg" width="300" alt="Тут повинна бути картинка">

## 🧰 Технології

- **[Rust](https://www.rust-lang.org/)** - мова, на якій весь проєкт написаний
- **[teloxide](https://github.com/teloxide/teloxide)** - фреймовк для створення Telegram-бота
- **[PostgreSQL](https://www.postgresql.org/)** - надійна база даних для зберігання всього необхідного
- **[diesel](https://diesel.rs/)** - ORM, що допомагає зручно працювати з БД
- **[OpenWeatherAPI](https://openweathermap.org/)** - звідки бот бере актуальний прогноз погоди
- **[fluent-bundle](https://crates.io/crates/fluent-bundle)** - для локалізації та підтримки різних мов


## 📦 Збірка та запуск

>  У вас повинен бути встановлений пакетний менеджер Cargo

### Встановлюємо секретики

1. Створюємо .env файл
    ```bash
    cp .env.example .env
    ```
   
2. Заповнюємо його

    ```bash
    TELEGRAM_TOKEN = "Токен_вашого_телеграм_бота"

    WEATHER_API_KEY = "Токен_від_open_weather_api"
    ```

### Побудова та запуск проєкту

```bash
# Білд
cargo build

# Запуск
cargo run
```

## 🛡 Ліцензія
Цей проєкт ліцензовано під [MIT License](./LICENSE).


## ✨ Автор

<table>
  <tr>
    <td>
      <a href="https://github.com/noinsts">
        <img src="https://avatars.githubusercontent.com/u/114863893?v=4" width="100px;" alt=""/>
        <br />
        <sub><b>noinsts</b></sub>
      </a>
    </td>
  </tr>
</table>
