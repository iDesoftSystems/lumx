use super::failure::{CreateDomainFailure, FindManyFailure, FindOneFailure, UpdateDomainFailure};

pub type CreateDomainResult<T> = Result<T, CreateDomainFailure>;

pub type UpdateDomainResult<T> = Result<T, UpdateDomainFailure>;

pub type FindManyDomainResult<T> = Result<T, FindManyFailure>;

pub type FindOneDomainResult<T> = Result<T, FindOneFailure>;
