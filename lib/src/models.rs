use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::*;

#[derive(Identifiable, Queryable, Clone)]
#[diesel(table_name = users)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub username: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(table_name = kanji, belongs_to(User))]
pub struct Kanji{
    pub id: i32,
    pub symbol: String,
    pub meaning: String,
    pub onyomi: Vec<Option<String>>,
    pub kunyomi: Vec<Option<String>>,
    pub description: Option<String>,
    pub vocab_refs: Vec<Option<String>>,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = kanji)]
pub struct NewKanji{
    pub symbol: String,
    pub meaning: String,
    pub onyomi: Vec<Option<String>>,
    pub kunyomi: Vec<Option<String>>,
    pub description: Option<String>,
    pub vocab_refs: Vec<Option<String>>,
    pub user_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(table_name = vocab, belongs_to(User))]
pub struct Vocab{
    pub id: i32,
    pub phrase: String,
    pub meaning: String,
    pub reading: Vec<Option<String>>,
    pub description: Option<String>,
    pub kanji_refs: Vec<Option<String>>,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = vocab)]
pub struct NewVocab{
    pub phrase: String,
    pub meaning: String,
    pub reading: Vec<Option<String>>,
    pub description: Option<String>,
    pub kanji_refs: Vec<Option<String>>,
    pub user_id: i32,
}