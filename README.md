# Full stack web application with CI and CD

![Build Status](https://github.com/wddler/note-taking-rs/actions/workflows/ci-build.yml/badge.svg) ![Audit Status](https://github.com/wddler/note-taking-rs/actions/workflows/security.yml/badge.svg) ![Deploy Status](https://github.com/wddler/note-taking-rs/actions/workflows/fly.yml/badge.svg)

This is an example of a full-stack web application with implemented DevOps practices: continuous integration and continuous deployment.

## Features

- actix-web framework (Rust programming language) is used for back-end.
- RESTful API is implemented (CRUD for data operations).
- JavaScript is used for front-end (eventually will be replaced by Rust-WebAssembly)
- CI pipeline: automated tests, linting, formatting, and security checks via GitHub Actions.
- CD pipeline: automated deployment on fly.io

---

Open in browser:

<https://note-taking.fly.dev/>

or

Run in terminal:

`$ cargo run`

or

Run in Docker:

`$ docker build -t note-taking .`

`$ docker run -p 8080:8080 -t note-taking`

Then open `http://localhost:8080/` in browser.

---

### Sample requests

Create note: `$ curl -XPOST localhost:8080/notes -H "Content-Type: application/json" -d '{"id":3, "text":"New note"}'`

Read note: `$ curl -XGET -i localhost:8080/notes/1`

Update note: `$ curl -XPUT localhost:8080/notes/1 -i -H "Content-Type: application/json" -d '{"id":1, "text":"Updated note"}'`

Delete note: `$ curl -XDELETE -i localhost:8080/notes/1`

Read all notes: `$ curl -XGET -i localhost:8080/notes`
