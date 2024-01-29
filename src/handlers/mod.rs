use crate::models::*;
use axum::{response::IntoResponse, Json};

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: question.title,
        description: question.description,
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    let questions: Vec<QuestionDetail> = vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "question_title".to_owned(),
        description: "question_description".to_owned(),
        created_at: "created_at".to_owned(),
    }];

    Json(questions)
}

pub async fn delete_question(Json(_question_uuid): Json<QuestionId>) {}

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "created_at".to_owned(),
    })
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {
    let answers: Vec<AnswerDetail> = vec![AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: question_id.question_uuid,
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }];

    Json(answers)
}

pub async fn delete_answer(Json(_answer_uuid): Json<AnswerId>) {}
