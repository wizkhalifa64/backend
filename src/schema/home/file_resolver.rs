use async_graphql::{Context, Object, Result, Upload};
use polars::prelude::*;
use serde_json::to_value;

use super::file_model::{FileBody, Fileresponse};

#[derive(Default)]
pub struct FileMutation;
#[Object]
impl FileMutation {
    async fn single_upload(
        &self,
        ctx: &Context<'_>,
        file: Upload,
        body: FileBody,
    ) -> Result<Fileresponse> {
        let upload = file.value(ctx).expect("Unable to read file");
        if let Some(file_type) = &upload.content_type {
            if file_type != "text/csv" {
                return Err("File type not supported".to_string().into());
            }
        }
        let df = CsvReader::new(upload.content)
            .truncate_ragged_lines(true)
            .infer_schema(Some(10))
            .has_header(true)
            .finish()?;
        let df_head = df.clone().head(Some(body.headcount));
        let df_describe = df.clone().describe(None)?;
        let response = Fileresponse {
            describe: to_value(df_describe).unwrap(),
            head: to_value(df_head).unwrap(),
        };
        Ok(response)
    }
}
