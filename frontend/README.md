# Frontend

## Design

The frontend is built with [Yew](https://yew.rs/).
No Javascript is used apart from the necessary glue to make things work.
The only extra build dependency is [Sass](https://sass-lang.com/), which is used for styling. The Sass files can be found in the `style` directory.

The `static` directory contains all files that don't need to be pre-processed. This includes HTML and localisation files.
Speaking of which, localisation is done using the [Fluent](https://www.projectfluent.org/) format. The files are located in `static/locale`.
