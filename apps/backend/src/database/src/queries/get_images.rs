use crate::queries::prelude::*;

#[cached(size = 100, time = 120, result = true)]
pub async fn exec(owner: i32, amount: i32, skip: i32) -> Result<Vec<Images>> {
    let row = get_tokio_postgres()
        .await
        .query(
            "SELECT created, id,vanity FROM images WHERE owner = $2 LIMIT $1 ORDER BY created OFFSET $3",
            &[&amount, &owner, &skip],
        )
        .await?;
    let mut new_rows: Vec<Images> = vec![];
    for row in row.iter() {
        new_rows.push(Images::from_row_ref(row).unwrap())
    }
    Ok(new_rows)
}