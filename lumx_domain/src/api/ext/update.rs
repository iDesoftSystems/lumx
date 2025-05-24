use crate::api::dtos::field::InvalidField;
use crate::{api, spi};

impl From<spi::failure::SaveRepoFailure> for api::failure::UpdateDomainFailure {
    fn from(err: spi::failure::SaveRepoFailure) -> Self {
        match err {
            spi::failure::SaveRepoFailure::Unknown(msg) => Self::Unknown(msg),
        }
    }
}

impl From<Vec<InvalidField>> for api::failure::UpdateDomainFailure {
    fn from(value: Vec<InvalidField>) -> Self {
        Self::InvalidFields(value)
    }
}

impl From<spi::failure::SelectRepoFailure> for api::failure::UpdateDomainFailure {
    fn from(failure: spi::failure::SelectRepoFailure) -> Self {
        match failure {
            spi::failure::SelectRepoFailure::Unknown(msg) => {
                api::failure::UpdateDomainFailure::Unknown(msg)
            }
        }
    }
}
