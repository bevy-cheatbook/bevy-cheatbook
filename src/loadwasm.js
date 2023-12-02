import '/wasm/restart-audio-context.js'

function init_wasm_cbexample(name) {
import("/wasm/" + name + ".js")
  .then((module) => {
    module.default().catch((error) => {
      if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
        throw error;
      }
    });
  })
}

function onclick_wasm_cbexample(name) {
  var button = document.getElementById("button_" + name);
  button.outerHTML = '<canvas id=\'' + "canvas_" + name + '\' width=\'640\' height=\'360\'></canvas>';
  init_wasm_cbexample(name);
}

const buttons = document.querySelectorAll('.button_wasm_cbexample');
buttons.forEach(button => {
  const name = button.id.replace('button_', '');
  button.addEventListener('click', () => {
    onclick_wasm_cbexample(name);
  });
});
