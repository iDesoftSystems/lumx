use std::ops::{Deref, DerefMut};

use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{error::WebError, state::AppState};

/// Extending the functionality of RequestParts
pub trait RequestPartsExt {
    /// get AppState
    fn get_app_state(&self) -> &AppState;

    /// get Component
    fn get_component<T: Clone + Send + Sync + 'static>(&self) -> Result<T, WebError>;
}

impl RequestPartsExt for Parts {
    fn get_app_state(&self) -> &AppState {
        self.extensions
            .get::<AppState>()
            .expect("extract app state from extension failed")
    }

    fn get_component<T>(&self) -> Result<T, WebError>
    where
        T: Clone + Send + Sync + 'static,
    {
        Ok(self.get_app_state().app.try_get_component()?)
    }
}

/// Extract the components registered by the plugin from AppState
pub struct Component<T: Clone>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequestParts<S> for Component<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, _s: &S) -> Result<Self, Self::Rejection> {
        parts.get_component::<T>().map(|c| Component(c))
    }
}

impl<T: Clone> Deref for Component<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for Component<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
