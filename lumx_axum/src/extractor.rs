use crate::state::AppState;
use crate::types::FailureReply;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use std::ops::Deref;
use std::sync::Arc;

/// Extending the functionality of RequestParams
pub trait RequestPartsExt {
    /// get AppState
    fn get_app_state(&self) -> &AppState;

    /// get Component
    fn get_component<T: Clone + Send + Sync + 'static>(&self) -> Option<Arc<T>>;
}

impl RequestPartsExt for Parts {
    fn get_app_state(&self) -> &AppState {
        self.extensions
            .get::<AppState>()
            .expect("extract app state from extension failed")
    }

    fn get_component<T: Clone + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.get_app_state().app.get_component::<T>()
    }
}

pub struct Component<T: Clone>(pub T);

impl<T: Clone> Deref for Component<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequestParts<S> for Component<T>
where
    T: Clone + Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<FailureReply>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        match parts.get_component::<T>() {
            None => {
                let message: String = format!(
                    "Component of type {0} does not exist in the registry",
                    std::any::type_name::<T>()
                );

                let failure: FailureReply = message.into();
                Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(failure)))?
            }
            Some(component) => Ok(Component(component.as_ref().clone())),
        }
    }
}
