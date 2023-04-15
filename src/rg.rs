use pgx::{opname, pg_operator, prelude::*, Aggregate};

// https://pt.wikipedia.org/wiki/C%C3%A9dula_de_identidade

extension_sql!(
    "\
CREATE TYPE rg AS (
    number bigint,
    digito char(1),
    estado char(2)
);",
    name = "create_composites",
    bootstrap
);

