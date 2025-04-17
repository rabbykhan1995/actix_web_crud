use actix_web::{
    Error, HttpResponse,
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::model::ResponseJson;

pub async fn is_admin_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error>
where
    B: MessageBody + 'static,
{
    if let Some(token) = req.cookie("token") {
        println!("Token is {:?}", token);
        if token.value() == "2144" {
            let res = next.call(req).await?;
            Ok(res.map_into_left_body())
        } else {
            Ok(req.into_response(
                HttpResponse::Unauthorized()
                    .json(ResponseJson {
                        msg: String::from("invalid token"),
                    })
                    .map_into_right_body(),
            ))
        }
    } else {
        Ok(req.into_response(
            HttpResponse::Unauthorized()
                .json(ResponseJson {
                    msg: String::from("un authorized request"),
                })
                .map_into_right_body(),
        ))
    }
}
