// use crate::connection::OdbcConnection;
// use crate::error::ProxyError;
// use anyhow::Result;
// use odbc_api::{buffers::TextRowSet, Cursor, ResultSetMetadata};
// use std::io::Write;

// const BATCH_SIZE: usize = 5000;

// pub struct QueryExecutor;

// impl QueryExecutor {
//     pub async fn execute<W: Write>(
//         conn: &OdbcConnection,
//         query: &str,
//         writer: &mut csv::Writer<W>,
//     ) -> Result<(), ProxyError> {
//         match conn.execute(query) {
//             Ok(Some(mut cursor)) => {
//                 // Write column names
//                 let headline: Vec<String> = cursor.column_names()?.collect::<Result<_, _>>()?;
//                 writer.write_record(headline)?;

//                 // Fetch and write rows
//                 let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
//                 let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

//                 while let Some(batch) = row_set_cursor.fetch()? {
//                     for row_index in 0..batch.num_rows() {
//                         let record = (0..batch.num_cols()).map(|col_index| {
//                             batch.at(col_index, row_index).unwrap_or(&[])
//                         });
//                         writer.write_record(record)?;
//                     }
//                 }
//             }
//             Ok(None) => {
//                 eprintln!("Query came back empty.");
//             }
//             Err(e) => return Err(ProxyError::Query(e.to_string())),
//         }
//         Ok(())
//     }
// }