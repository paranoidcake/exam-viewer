# exam-viewer

Web server to provide various views of Leaving Certificate exam papers and marking schemes.

[Warp](https://github.com/seanmonstar/warp) is used to serve pages which are rendered server-side using [`handlebars`](https://github.com/sunng87/handlebars-rust).
CSS is handled using [Bulma](https://bulma.io), and is the only frontend dependency.

Development wise, the goal is to reduce client-side bloat by using server-side rendering and as little Javascript as possible to keep page loads quick and small.

## Installation
0. Install `rust`
1. Run `cargo build`

## Running the webserver
1. Run `cargo run`

Compilation may take a while when you update the rust code, but once compiled it launches much faster. As `dev_mode` is enabled for handlebars, you don't need to re-run the server between changes to the frontend.

On your first time, you should uncomment and run `lib::scraper::scrape_exams()` once, which should begin downloading the exam papers/marking scheme pdfs.
It is currently a fairly simple web scraper and will be replaced some day for reliability.
Currently, it is very slow to avoid being timed out and if interrupted must start downloading from the beginning, overwriting already downloaded files.
For test purposes I only wait for a few subjects to download.