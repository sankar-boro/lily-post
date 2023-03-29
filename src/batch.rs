#[macro_export]
macro_rules! create_batch {
    ( $( $x:expr ),* ) => {
        {
            let mut batch: Batch = Default::default();
            $(
                batch.append_statement($x);
            )*
            batch
        }
    };
}