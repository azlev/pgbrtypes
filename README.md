# pgbrtypes

An experiment to create postgres extensions using Rust [`pgx`](https://github.com/tcdi/pgx) infrastructure.

Also, it's another project to explore brazilian document numbers (CNPJ, CPF, PIS, RG) by creating strong types to each one. The postgres type system will grant:

- check digits validation
- int64 storage (better performance and indexing)
- string conversion and mask

