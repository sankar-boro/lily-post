use deadpool_postgres::Pool;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::error::Error;
use super::model::CreateNode;

pub async fn append(
    app: web::Data<Pool>,
    payload: web::Json<CreateNode>,
    session: Session
) 
-> Result<HttpResponse, Error> 
{
  super::create::create(app, payload, session).await
}
