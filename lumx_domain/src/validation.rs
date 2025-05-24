use validator::Validate;

use crate::api::dtos::field::InvalidField;

pub struct Validable;

impl Validable {
    pub fn try_validate<T>(params: &T) -> Result<(), Vec<InvalidField>>
    where
        T: Validate,
    {
        match params.validate() {
            Ok(()) => Ok(()),
            Err(err) => Err(Self::to_domain(err)),
        }
    }

    fn to_domain(errs: validator::ValidationErrors) -> Vec<InvalidField> {
        let mut val_errors: Vec<InvalidField> = errs
            .field_errors()
            .into_iter()
            .map(|error| InvalidField::new(error.0.to_string(), error.1[0].code.to_string()))
            .collect();

        val_errors.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

        val_errors
    }
}

#[cfg(test)]
mod tests {
    use validation::Validate;

    use crate::validation;

    #[derive(Validate, Debug)]
    struct ValidateMe {
        #[validate(length(min = 1))]
        pub name: String,
    }

    #[test]
    fn it_pass_validation() {
        let to_validate = ValidateMe {
            name: "Hello".to_string(),
        };
        let result = validation::Validable::try_validate(&to_validate);
        assert!(result.is_ok())
    }

    #[test]
    fn it_failed_validation() {
        let to_validate = ValidateMe {
            name: "".to_string(),
        };
        let result = validation::Validable::try_validate(&to_validate);

        assert!(result.is_err())
    }
}
