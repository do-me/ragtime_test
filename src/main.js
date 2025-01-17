const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let embedInputEl;
let embedMsgEl;


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function initRag() {
    const modelPaths = {
        qa_model: "models/Phi-3-mini-128k-instruct/ggml-model-q8_0.gguf"
    };
    try {
       await invoke("init_rag", { modelPaths: modelPaths })
        embedMsgEl.textContent = "Models loaded";
    } catch (e) {
        embedMsgEl.textContent = e;
    }
}

async function getEmbedding() {
   try {
      const result = await invoke("get_embedding", { text: embedInputEl.value })
      embedMsgEl.textContent = JSON.stringify(result);
   }
   catch (e) {
      embedMsgEl.textContent = e;
   }
}



window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
    embedInputEl = document.querySelector("#embed-input");
    embedMsgEl = document.querySelector("#embed-msg");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

   document.querySelector("#init-rag").addEventListener("click", () => {
        initRag();
   })
   document.querySelector("#embed-form").addEventListener("submit", (e) => {
    e.preventDefault();
     getEmbedding();
   });
});