use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct TagRow {
    pub id: i32,
    pub name: String,
}


#[derive(Debug, FromRow)]
pub struct ArticleTag {
    pub id: i32,
    pub article_id: i32,
    pub tag_id: i32,
}