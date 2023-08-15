use std::sync::Arc;

use llm_wrapper::llm::{self, InferenceRequest, InferenceSessionConfig};
use tokio::sync::Mutex;

/// Emergency wrapper
pub async fn infer_text(model: &dyn llm_wrapper::llm::Model, prompt: String) -> String {
    let infered = Arc::new(Mutex::new(vec![]));

    let infered_clone = Arc::clone(&infered);
    tokio::task::block_in_place(move || {
        let mut session = model.start_session(InferenceSessionConfig {
            n_batch: 2,
            n_threads: 1,
            ..Default::default()
        });

        session
            .infer::<std::convert::Infallible>(
                model,
                &mut rand::thread_rng(),
                &InferenceRequest {
                    prompt: ((&prompt).into()),
                    parameters: &llm::InferenceParameters::default(),
                    play_back_previous_tokens: false,
                    maximum_token_count: Some(64),
                },
                &mut llm::OutputRequest::default(),
                |r| match r {
                    llm::InferenceResponse::PromptToken(t)
                    | llm::InferenceResponse::InferredToken(t) => {
                        tracing::info!(t);

                        {
                            let infered = infered_clone.clone();
                            infered.blocking_lock().push(t);
                        }

                        Ok(llm::InferenceFeedback::Continue)
                    }
                    _ => Ok(llm::InferenceFeedback::Continue),
                },
            )
            .unwrap();
    });

    let infered_clone = Arc::clone(&infered);
    let joined = infered_clone.lock().await.join("");
    joined
}
