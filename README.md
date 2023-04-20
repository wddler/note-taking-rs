# Full stack web application with CI and CD

![Build Status](https://github.com/wddler/note-taking-rs/actions/workflows/ci-build.yml/badge.svg) ![Audit Status](https://github.com/wddler/note-taking-rs/actions/workflows/security.yml/badge.svg) ![Deploy Status](https://github.com/wddler/note-taking-rs/actions/workflows/fly.yml/badge.svg)

This is an example of a full-stack web application with implemented DevOps practices: continuous integration and continuous deployment.

## Features

- actix-web framework (Rust programming language) is used for back-end.
- RESTful API is implemented (CRUD for data operations).
- JavaScript is used for front-end (eventually will be replaced by Rust-WebAssembly)
- CI pipeline: automated tests, linting, formatting, and security checks via GitHub Actions.
- CD pipeline: automated deployment on fly.io

Open in browser:

<https://note-taking.fly.dev/>

Run in terminal:

`$ cargo run`

Run in Docker:

`$ docker build -t note-taking .`

`$ docker run -p 8080:8080 -t note-taking`
