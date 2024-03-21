[![Rust](https://github.com/hwndept/rust-bootcamp-qanda-api-app/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/hwndept/rust-bootcamp-qanda-api-app/actions/workflows/ci.yml)

# rust-bootcamp-qanda-api-app

This project is part of a rust boot camp course practical task. It's a API app - kind of a question & answers, _like StackOverflow_.

The project implements the requirements defined in [bootcamp](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/2.%20API-axum/Problem).

## How to use

1. Run a local instance of PostgreSQL:

The next script runs database instance using docker. You can avoid this step if you have another database instance.

```sh
./scripts/run-db.sh
```

_Hit `CTRL+C` to stop._

2. Create `.env` file in the root of the project and specify DATABASE_URL, e.g.:

```
DATABASE_URL=postgresql://user:password@localhost/default
```

3. Initiate database.

```sh
# Install sqlx-cli
cargo install sqlx-cli
# Apply migrations
sqlx migrate run
```

3. Run the command below (_it compiles the code and executes it_).

```sh
RUST_LOG=info cargo run
```

The app will run HTTP server exposed on [http://127.0.0.1:8000/](http://0.0.0.0:8000/)

## API

There is preset file, [Bootcamp API Q&A Preset](./docs/api/BootCamp%20API%20Q&A%20-%20Preset.json), that can be loaded to Thunder Client to play with API.

1. `GET /questions` - _Lists all questions_:

```sh
curl --request GET \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json'
```

2. `POST /question` - _Creates a question_:

```sh
curl --request POST \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json' \
  --data '{
    "title": "Title",
    "description": "Description"
}'
```

3. `DELETE /question` - _Deletes a question_:

```sh
curl --request DELETE \
  --url http://localhost:8000/questions/<QUESTION_UUID> \
  --header 'Accept: application/json'
```

4. `GET /answers` - _Lists all answers_:

```sh
curl --request GET \
  --url http://localhost:8000/answers/<QUESTION_UUID> \
  --header 'Accept: application/json'
```

5. `POST /answers` - _Creates an answer_:

```sh
curl --request POST \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "[UUID of a created question]",
    "content": "Content"
}'
```

6. `DELETE /answer` - _Deletes an answer_:

```sh
curl --request DELETE \
  --url http://localhost:8000/answers/<ANSWER_UUID>
```
