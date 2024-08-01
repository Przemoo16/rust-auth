document.addEventListener("DOMContentLoaded", (event) => {
  document.body.addEventListener('htmx:beforeSwap', function(evt) {
    if ([401, 409, 422].includes(evt.detail.xhr.status)) {
      evt.detail.shouldSwap = true;
      evt.detail.isError = false;
    }
  });
})
