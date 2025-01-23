use super::dtos::ProfileDto;
use crate::Context;
use domain::Read;
use rspc::{Router, RouterBuilder};

pub fn profile_router() -> RouterBuilder<Context> {
    Router::<Context>::new()
        .query("read", |t| {
            t(|ctx, username: String| async move {
                ctx.profile_service
                    .read(username)
                    .await
                    .map(ProfileDto::from)
                    // TODO: better error handling
                    .map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })
            })
        })
        .query("list", |t| {
            t(|ctx, _: ()| async move {
                ctx.profile_service
                    .read(())
                    .await
                    .map(|profiles| {
                        profiles
                            .into_iter()
                            .map(ProfileDto::from)
                            .collect::<Vec<ProfileDto>>()
                    })
                    // TODO: better error handling
                    .map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })
            })
        })
}
