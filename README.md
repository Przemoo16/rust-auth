# TODO:

- Add loader for pages if they load too slow.
  Currently loaders are only added for signup and signin redirections so the submit button is not re-enabled again when redirection is happening.
  Issue with applying loaders to all pages is that if the page is loaded super fast, there is flickering screen.
  Adding transition-delayed to e.g. 200ms solves this but then the issue with visible submit button re-enabling is back.
- Use [HTTP caching](https://developer.mozilla.org/en-US/docs/Web/HTTP/Caching#common_caching_patterns):
  Add Etag and Last-Modified headers to all resources
  Add hash to css and js resources to make sure that the browser always gets the latest version of the resource
  Support responding with 304 if resource has not changed
