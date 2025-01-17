use ragtime::{
    llama,
    QaModel,
    phi3::llama::Phi3,
    FormattedPrompt
};
use tauri::State;
use std::sync::Arc;
use llama_cpp_2::llama_backend::LlamaBackend;
use anyhow::{Result, anyhow};
use compact_str::CompactString;

// Define a struct to hold your ragtime state.
#[derive(Debug)]
pub struct RagState {
    rag: tokio::sync::Mutex<Phi3>,
}

// make sure that RagState is threadsafe
unsafe impl Send for RagState {}
unsafe impl Sync for RagState {}

// Define a struct that holds the paths to the models to make things simpler
#[derive(Debug, serde::Deserialize)]
pub struct ModelArgs {
    qa_model: String,
}

#[tauri::command]
async fn init_rag(
    state: State<'_, RagState>,
    model_paths: ModelArgs
) -> Result<(), String> {
    let backend = Arc::new(match LlamaBackend::init() {
        Ok(be) => be,
        Err(e) => return Err(format!("{e:?}")),
    });
    let model_paths = ModelArgs {
        qa_model: format!("./{}", model_paths.qa_model)
    };
    let mut rag = state.rag.lock().await;
    match Phi3::new(
            backend,
            llama::Args::default()
                .with_threads(8)
                .with_gpu_layers(1)
                .with_model(model_paths.qa_model)
        ) {
        Ok(r) => *rag = r,
        Err(e) => return Err(format!("{e:?}"))
    };
    Ok(())
}


#[tauri::command]
async fn get_embedding(
    state: State<'_, RagState>,
    text: String,
) -> Result<Vec<f32>, String> {
    use std::fmt::Write;
    let mut rag = state.rag.lock().await;
        let mut prompt = <Phi3 as QaModel>::Prompt::new();
        write!(prompt.user(), "{text}").map_err(|e| format!("{e:?}"))?;

        let tokens = match rag.ask(prompt.finalize().map_err(|e| format!("{e:?}"))?, None).map_err(|e| format!("{e:?}"))?
          .next().transpose()
            {
                Some(tok) => {
                let s =  tok.map_err(|e| format!("{e:?}"))?;
                   match rag.inner.model.str_to_token(&s.as_str(), llama_cpp_2::model::AddBos::Never) {
                       Ok(t) => t,
                       Err(e) => return Err(format!("{e:?}")),
                   }
                },
            None => return Err(String::from("No tokens were generated")),
        };


      let emb = match rag.inner.as_mut().ctx().embeddings_ith(tokens.last().unwrap().0 as i32) {
        Ok(emb) => emb,
        Err(e) => return Err(format!("{e:?}"))
    };

   Ok(emb)
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     let rag = RagState {
        rag: tokio::sync::Mutex::new(
            Phi3::new(
                 Arc::new(LlamaBackend::init().unwrap()),
                   llama::Args::default()
                       .with_threads(8)
                       .with_gpu_layers(1)
                       .with_model("./models/Phi-3-mini-128k-instruct/ggml-model-q8_0.gguf")
                ).unwrap(),
        ),
    };
    tauri::Builder::default()
         .plugin(tauri_plugin_opener::init())
        .manage(rag)
        .invoke_handler(tauri::generate_handler![greet, init_rag, get_embedding])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}