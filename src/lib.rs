pub use pgrx::prelude::*;

pgrx::pg_module_magic!();

mod setup;

#[pg_extern]
fn hello_pg_auto_dw() -> &'static str {
    "Hello, pg_auto_dw"
}

#[pg_extern]
fn evaluate() -> Result<
    TableIterator<
        'static,
        (
            name!(schema_name, Result<Option<String>, pgrx::spi::Error>),
            name!(table_name, Result<Option<String>, pgrx::spi::Error>),
            name!(column_name, Result<Option<String>, pgrx::spi::Error>),
            name!(column_cat, Result<Option<String>, pgrx::spi::Error>),
            name!(confidence_level, Result<Option<String>, pgrx::spi::Error>),
            name!(is_overridden, Result<Option<bool>, pgrx::spi::Error>)
        )
    >,
    spi::Error,
> {
    let schema = "public";
    // let table = "customer";

    let query_string = format!(r#"
        SELECT schema_name, 
            table_name, 
            column_name, 
            column_cat, 
            confidence_level, 
            is_overridden 
        FROM auto_dw.table_column_cat
        WHERE 
            schema_name = '{}'
        "#, schema);
    
    let query: &str = query_string.as_str();

    info!("Evaluation of TABLE customer");
    Spi::connect(|client| {
        Ok(client
            .select(query, None, None)?
            .map(|row| (
                row["schema_name"].value(), 
                row["table_name"].value(), 
                row["column_name"].value(),
                row["column_cat"].value(),
                row["confidence_level"].value(),
                row["is_overridden"].value())
            )
            .collect::<Vec<_>>())
    })
    .map(TableIterator::new)
}

#[pg_extern]
fn evaluate_table(table: &str) -> Result<
    TableIterator<
        'static,
        (
            name!(schema_name, Result<Option<String>, pgrx::spi::Error>),
            name!(table_name, Result<Option<String>, pgrx::spi::Error>),
            name!(column_name, Result<Option<String>, pgrx::spi::Error>),
            name!(column_cat, Result<Option<String>, pgrx::spi::Error>),
            name!(confidence_level, Result<Option<String>, pgrx::spi::Error>),
            name!(is_overridden, Result<Option<bool>, pgrx::spi::Error>)
        )
    >,
    spi::Error,
> {
    let schema = "public";
    // let table = "customer";

    let query_string = format!(r#"
        SELECT schema_name, 
            table_name, 
            column_name, 
            column_cat, 
            confidence_level, 
            is_overridden 
        FROM auto_dw.table_column_cat
        WHERE 
            schema_name = '{}' AND 
            table_name = '{}'
        "#, schema, table);
    
    let query: &str = query_string.as_str();

    info!("Evaluation of TABLE customer");
    Spi::connect(|client| {
        Ok(client
            .select(query, None, None)?
            .map(|row| (
                row["schema_name"].value(), 
                row["table_name"].value(), 
                row["column_name"].value(),
                row["column_cat"].value(),
                row["confidence_level"].value(),
                row["is_overridden"].value())
            )
            .collect::<Vec<_>>())
    })
    .map(TableIterator::new)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_pg_auto_dw() {
        assert_eq!("Hello, pg_auto_dw", crate::hello_pg_auto_dw());
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
