use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

/// Simple CLI to interface with Leetcode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Download a problem locally
    Download { id: u32 },
    /// Submit a solution to Leetcode's servers
    Submit { id: u32 },
    /// List available problems
    List,
}

#[derive(Serialize, Debug)]
struct GraphQLRequest {
    query: String,
    variables: QuestionDetailVariables,
    #[serde(rename = "operationName")]
    operation_name: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QuestionDetailVariables {
    title_slug: String,
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
struct Question {
    content: String,
}

fn download(id: u32) {
    println!("Downloading {id} but really two-sum");

    let client = reqwest::blocking::Client::new();

    let new_post = GraphQLRequest {
        query: String::from(
            "query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { content stats codeDefinition sampleTestCase exampleTestcases enableRunCode metaData translatedContent } }",
        ),
        variables: QuestionDetailVariables {
            title_slug: String::from("two-sum"),
        },
        operation_name: String::from("getQuestionDetail"),
    };
    let res = client
        .post("https://leetcode.com/graphql")
        .json(&new_post)
        .send();
    if let Ok(res) = res {
        if res.status().is_success() {
            let response: GraphQLResponse = res.json().unwrap();
            let content = response.data.question.content;
            println!("{}", content);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Download { id }) => {
            download(id);
        }
        Some(Commands::Submit { id }) => {
            println!("Submitting {id}");
        }
        Some(Commands::List) => {
            println!("Available IDs:");
            for i in 0..10 {
                println!("ID: {i}");
            }
        }
        None => {}
    }
}
