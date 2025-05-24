use crate::api;
use crate::spi;

impl From<spi::failure::SelectRepoFailure> for api::failure::FindOneFailure {
    fn from(failure: spi::failure::SelectRepoFailure) -> Self {
        match failure {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                api::failure::FindOneFailure::Unknown(msg)
            }
        }
    }
}

impl From<spi::failure::SelectRepoFailure> for api::failure::FindManyFailure {
    fn from(value: spi::failure::SelectRepoFailure) -> Self {
        match value {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                api::failure::FindManyFailure::Unknown(msg)
            }
        }
    }
}
