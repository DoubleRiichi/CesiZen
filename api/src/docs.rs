#[derive(OpenApi)]
#[openapi(
    paths(
        //Articles
        crate::modules::article::handler::get_article_by_id,
        crate::modules::article::handler::search_article,
        crate::modules::article::handler::create_article,
        crate::modules::article::handler::update_article,
        crate::modules::article::handler::delete_article,
        //Users
        crate::modules::user::handler::get_user_by_id,
        crate::modules::user::handler::search_user,
        crate::modules::user::handler::create_user,
        crate::modules::user::handler::update_user,
        crate::modules::user::handler::delete_user,
        crate::modules::user::handler::login,

        //Tags
        crate::modules::tag::handler::get_tag_by_id,
        crate::modules::tag::handler::create_tag,
        crate::modules::tag::handler::delete_tag,
        crate::modules::tag::handler::all_tag,
        //Feelings
        crate::modules::feeling::handler::get_feeling_by_id,
        crate::modules::feeling::handler::create_feeling,
        crate::modules::feeling::handler::search_feeling,
        crate::modules::feeling::handler::delete_feeling,

        // add the rest...
    ),
    components(
        schemas(
            crate::modules::article::dto::ArticleGet,
            crate::modules::article::dto::ArticleCreate,
            crate::modules::article::dto::ArticleUpdate,
            crate::modules::article::dto::ArticleSearchParams,

            crate::modules::user::dto::UserGet,
            crate::modules::user::dto::UserSearchParams,
            crate::modules::user::dto::UserCreate,
            crate::modules::user::dto::UserUpdate,
            crate::modules::user::dto::UserSearchParams,

            crate::modules::tag::dto::TagGet,
            crate::modules::tag::dto::TagCreate,

            crate::modules::feeling::dto::FeelingGet,
            crate::modules::feeling::dto::FeelingCreate,
            crate::modules::feeling::dto::FeelingSearchParams
        )
    ),
    tags(
        (name = "article", description = "Article management endpoints"),
        (name = "user", description = "User management"),
        (name = "tag", description = "Tags management"),
        (name = "feeling", description = "Feeling management"),

    ),
    info(
        title = "Cesizen API",
        description = "My awesome Rust + Axum + PostgreSQL API",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;

use utoipa::OpenApi;
