use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::PathBuf,
    process::{ExitCode, Termination},
};

pub enum ToolResult<T> {
    Ok(T),
    Err(ToolError),
}

impl<T> Termination for ToolResult<T> {
    fn report(self) -> ExitCode {
        match self {
            Self::Ok(_) => ExitCode::SUCCESS,
            Self::Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}

#[derive(Debug)]
pub enum ToolError {
    Other(String),
}

impl std::error::Error for ToolError {}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Other(e) => write!(f, "Some other error: {}", e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JeopardyGame {
    categories: Vec<JeopardyCategory>,
}

impl JeopardyGame {
    pub fn new() -> Self {
        JeopardyGame {
            categories: Vec::new(),
        }
    }
}

impl JeopardyGame {
    /// Check whether a game has 5 categories and every category has 5 answers
    fn is_valid(&self) -> bool {
        self.categories.len() == 5 && self.categories.iter().all(|c| c.is_valid())
    }

    /// Randomly choose two answers to be a Double Jeopardy
    fn double_jeopardy(&mut self) {
        let mut rng = rand::thread_rng();
        let mut indices = (0..self.categories.len()).collect::<Vec<_>>();
        indices.shuffle(&mut rng);
        for i in indices.iter().take(2) {
            self.categories[*i].double_jeopardy();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JeopardyCategory {
    name: String,
    answers: Vec<JeopardyAnswer>,
}

impl JeopardyCategory {
    /// Check whether a category has 5 answers
    fn is_valid(&self) -> bool {
        self.answers.len() == 5 && self.answers.iter().all(|a| a.is_valid())
    }

    /// Randomly choose two answers to be a Double Jeopardy
    fn double_jeopardy(&mut self) {
        let mut rng = rand::thread_rng();
        let mut indices = (0..self.answers.len()).collect::<Vec<_>>();
        indices.shuffle(&mut rng);
        for i in indices.iter().take(2) {
            match &mut self.answers[*i] {
                JeopardyAnswer::Text {
                    answer: _,
                    question: _,
                    double_jeopardy,
                } => *double_jeopardy = true,
                JeopardyAnswer::Image {
                    question: _,
                    image: _,
                    double_jeopardy,
                } => *double_jeopardy = true,
                JeopardyAnswer::Audio {
                    question: _,
                    audio: _,
                    double_jeopardy,
                } => *double_jeopardy = true,
                JeopardyAnswer::Video {
                    question: _,
                    video: _,
                    double_jeopardy,
                } => *double_jeopardy = true,
            };
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum JeopardyAnswer {
    Text {
        answer: String,
        question: String,
        double_jeopardy: bool,
    },
    Image {
        question: String,
        image: PathBuf,
        double_jeopardy: bool,
    },
    Audio {
        question: String,
        audio: PathBuf,
        double_jeopardy: bool,
    },
    Video {
        question: String,
        video: PathBuf,
        double_jeopardy: bool,
    },
}

impl JeopardyAnswer {
    /// Check whether an answer is valid
    fn is_valid(&self) -> bool {
        match self {
            Self::Text { .. } => true,
            Self::Image { .. } => true,
            Self::Audio { .. } => true,
            Self::Video { .. } => true,
        }
    }
}

pub fn show(prefix: Option<String>) -> ToolResult<()> {
    println!("{:?}", prefix);
    let mut jeopardy_game = JeopardyGame::new();
    println!("{:?}", jeopardy_game);
    println!("{:?}", jeopardy_game.is_valid());
    jeopardy_game.double_jeopardy();
    ToolResult::Err(ToolError::Other("Some error".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn jeopardy_game_strategy()(
            categories in prop::collection::vec(jeopardy_category_strategy(), 5),
        ) -> JeopardyGame {
            JeopardyGame { categories }
        }
    }

    prop_compose! {
        fn jeopardy_category_strategy()(
            name in any::<String>(),
            answers in prop::collection::vec(jeopardy_answer_strategy(), 5),
        ) -> JeopardyCategory {
            JeopardyCategory { name, answers }
        }
    }

    prop_compose! {
        fn path_buf_strategy()(
            path in prop::collection::vec(any::<String>(), 1..10),
        ) -> PathBuf {
            PathBuf::from(path[0].as_str())
        }
    }

    fn jeopardy_answer_strategy() -> impl Strategy<Value = JeopardyAnswer> {
        prop_oneof![
            (any::<String>(), any::<String>(), any::<bool>()).prop_map(
                |(answers, question, dj)| {
                    JeopardyAnswer::Text {
                        answer: answers,
                        question: question,
                        double_jeopardy: dj,
                    }
                }
            ),
            (any::<String>(), path_buf_strategy(), any::<bool>()).prop_map(
                |(answers, path, dj)| {
                    JeopardyAnswer::Image {
                        question: answers,
                        image: path,
                        double_jeopardy: dj,
                    }
                }
            ),
            (any::<String>(), path_buf_strategy(), any::<bool>()).prop_map(
                |(answers, path, dj)| {
                    JeopardyAnswer::Audio {
                        question: answers,
                        audio: path,
                        double_jeopardy: dj,
                    }
                }
            ),
            (any::<String>(), path_buf_strategy(), any::<bool>()).prop_map(|(answer, path, dj)| {
                JeopardyAnswer::Video {
                    question: answer,
                    video: path,
                    double_jeopardy: dj,
                }
            }),
        ]
    }

    proptest! {
        #[test]
        fn jeopardy_game_is_valid(jeopardy_game in jeopardy_game_strategy()) {
            assert!(jeopardy_game.is_valid());
        }
    }

    proptest! {
        #[test]
        fn jeopardy_category_is_valid(jeopardy_category in jeopardy_category_strategy()) {
            assert!(jeopardy_category.is_valid());
        }
    }

    proptest! {
        #[test]
        fn jeopardy_answer_is_valid(jeopardy_answer in jeopardy_answer_strategy()) {
            assert!(jeopardy_answer.is_valid());
        }
    }
}
