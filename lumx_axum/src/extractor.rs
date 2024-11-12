use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct FailureReply {
    message: String,
    errors: Vec<ValidationFailure>,
}

#[derive(Debug, Serialize)]
pub struct ValidationFailure {
    pub field: String,
    pub error: String,
}

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

        // match self.get_app_state().app.get_component::<T>() {
        //     None => {
        //         let message = format!("Component of type {0} does not exist in the registry", std::any::type_name::<T>());
        //         Err(WebFailure::ComponentNotExist(message))
        //     }
        //     Some(component) => {
        //         Ok(T::clone(&component))
        //     }
        // }
    }
}

pub struct Component<T: Clone>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequestParts<S> for Component<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Rejection = (StatusCode, axum::Json<FailureReply>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        match parts.get_component::<T>() {
            None => {
                let message = format!(
                    "Component of type {0} does not exist in the registry",
                    std::any::type_name::<T>()
                );
                let failure = FailureReply {
                    message,
                    errors: Vec::new(),
                };
                Err((StatusCode::INTERNAL_SERVER_ERROR, axum::Json(failure)))?
            }
            Some(component) => Ok(Component(component.as_ref().clone())),
        }
    }
}
