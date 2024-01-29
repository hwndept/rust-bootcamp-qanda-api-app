# rust-bootcamp-qanda-api-app

This project is part of a rust boot camp course practical task. It's a API app - kind of a question & answers, _like StackOverflow_.

The project implements the requirements defined in [bootcamp](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/2.%20API-axum/Problem).

## How to use

Run the command below (_it compiles the code and executes it_).

```sh
cargo run
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
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
    "title": "Title",
    "description": "Description"
}'
```

3. `DELETE /question` - _Deletes a question_:

```sh
curl --request DELETE \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "[UUID of a created question]"
}'
```

4. `GET /answers` - _Lists all answers_:

```sh
curl --request GET \
  --url http://localhost:8000/answers \
  --header 'Accept: application/json' \
  --data '{
    "question_uuid": "[UUID of a created question]"
}'
```

5. `POST /answer` - _Creates an answer_:

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
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
    "answer_uuid": "[UUID of a created answer]"
}'
```

## To do

- [ ] Rename API resources to plural form.
- [ ] Move GET requests body params to path params.
