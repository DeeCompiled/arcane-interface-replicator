const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  
  document.querySelector("#directory-select").addEventListener("click", async ()  => {
    let selectedDirectory = await invoke("select_directory");
    document.querySelector("#available-characters-list").innerHTML = selectedDirectory;
  });

});
