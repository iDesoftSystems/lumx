use crate::api::dtos::field::InvalidField;
use crate::{api, spi};

impl From<spi::failure::SaveRepoFailure> for api::failure::CreateDomainFailure {
    fn from(err: spi::failure::SaveRepoFailure) -> Self {
        match err {
            spi::failure::SaveRepoFailure::Unknown(msg) => Self::Unknown(msg),
        }
    }
}

impl From<Vec<InvalidField>> for api::failure::CreateDomainFailure {
    fn from(value: Vec<InvalidField>) -> Self {
        Self::InvalidFields(value)
    }
}

impl From<spi::failure::SelectRepoFailure> for api::failure::CreateDomainFailure {
    fn from(failure: spi::failure::SelectRepoFailure) -> Self {
        match failure {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                api::failure::CreateDomainFailure::Unknown(msg)
            }
        }
    }
}
