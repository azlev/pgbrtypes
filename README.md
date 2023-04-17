# pgbrtypes

An experiment to create postgres extensions using Rust [`pgrx`](https://github.com/tcdi/pgrx) infrastructure.

Also, it's another project to explore brazilian document numbers (CNPJ, CPF, PIS, RG) by creating strong types to each one. The postgres type system will grant:

- check digits validation
- int64 storage (better performance and indexing)
- string conversion and mask

## Installation

```bash
cargo pgrx package # pg_config must be on PATH

psql
CREATE EXTENSION pgbrtypes;
SELECT CNPJ '191';
        cnpj        
--------------------
 00.000.000/0001-91
(1 row)
```

