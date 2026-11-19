use directories::ProjectDirs;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Debug)]
struct GraphQLRequest<'a> {
    query: String,
    variables: QuestionDetailVariables<'a>,
    #[serde(rename = "operationName")]
    operation_name: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QuestionDetailVariables<'a> {
    title_slug: &'a str,
}

#[derive(Deserialize)]
struct GraphQLResponse {
    data: QuestionData,
}

#[derive(Deserialize)]
struct QuestionData {
    question: Question,
}

#[derive(Deserialize)]
pub struct Question {
    pub content: String,
}

#[derive(Deserialize)]
pub struct RestAPIResponse {
    stat_status_pairs: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct Entry {
    stat: Stat,
}

#[derive(Deserialize)]
pub struct Stat {
    frontend_question_id: u32,
    #[serde(rename = "question__title_slug")]
    question_title_slug: String,
}

pub enum DownloadError {
    Request(reqwest::Error),
    UnknownId(u32),
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        DownloadError::Request(err)
    }
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownId(id) => write!(f, "unknown problem ID {id}"),
            Self::Request(error) => write!(f, "{error}"),
        }
    }
}

pub fn download(id: u32) -> Result<Question, DownloadError> {
    let problems = get_all_questions()?;
    let slug = problems.get(&id).ok_or(DownloadError::UnknownId(id))?;
    let client = reqwest::blocking::Client::builder().build()?;

    let new_post = GraphQLRequest {
        query: String::from(
            "query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { content stats codeDefinition sampleTestCase exampleTestcases enableRunCode metaData translatedContent } }",
        ),
        variables: QuestionDetailVariables { title_slug: slug },
        operation_name: String::from("getQuestionDetail"),
    };
    let response: GraphQLResponse = client
        .post("https://leetcode.com/graphql")
        .json(&new_post)
        .send()?
        .error_for_status()?
        .json()?;

    Ok(response.data.question)
}

pub fn get_all_questions() -> Result<HashMap<u32, String>, reqwest::Error> {
    let cache_path = ProjectDirs::from("com", "danong", "leetcode-cli")
        .map(|dirs| dirs.cache_dir().join("problem_map_cache.json"));

    if let Some(ref path) = cache_path {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(problems) = serde_json::from_str(&content) {
                return Ok(problems);
            }
        }
    }
    let client = reqwest::blocking::Client::builder().build()?;
    let body: RestAPIResponse = client
        .get("https://leetcode.com/api/problems/all/")
        .header(USER_AGENT, "leetcode-cli/0.1.0")
        .send()?
        .error_for_status()?
        .json()?;
    let problems = body
        .stat_status_pairs
        .into_iter()
        .map(|entry| {
            (
                entry.stat.frontend_question_id,
                entry.stat.question_title_slug,
            )
        })
        .collect();
    if let Some(path) = cache_path {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string(&problems) {
            let _ = fs::write(path, json);
        }
    }
    Ok(problems)
}
