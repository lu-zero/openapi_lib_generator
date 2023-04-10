//! YAML config file generation

use crate::{
  cli::Cli,
  fs::write,
  generate::{errors::ParameterError, makefiles::MakefileEnv},
  testing,
};
use fs_err::tokio as fs;
use serde::{Deserialize, Serialize};
use serde_yaml::Error as SerdeYAMLError;
use std::io::Error as IOError;
use thiserror::Error;
/// Errors that can happen with yaml generation
#[derive(Debug, Error)]
pub enum YAMLGenerationError {
  #[error(transparent)]
  IOError(#[from] IOError),
  #[error(transparent)]
  SerdeYAMLError(#[from] SerdeYAMLError),
  #[error(transparent)]
  ParameterError(#[from] ParameterError),
}

/// Rust OpenAPI Generator Configs  
///
/// - See: <https://openapi-generator.tech/docs/generators/rust/>
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OpenAPIRustGeneratorConfigs {
  /// Use best fitting integer type where minimum or maximum is set (default false)
  pub bestFitInt: bool,
  /// Suffix that will be appended to all enum names.
  pub enumNameSuffix: String,
  /// Hides the generation timestamp when files are generated. (default true)
  pub hideGenerationTimestamp: bool,
  /// library template (sub-template) to use.(hyper or reqwest, default reqwest)
  pub library: String,
  /// Rust package name (convention: lowercase). (default openapi)
  pub packageName: String,
  /// Rust package version.(default 1.0.0)
  pub packageVersion: String,
  /// Prefer unsigned integers where minimum value is >= 0(default false)
  pub preferUnsignedInt: bool,
  /// If set, generate async function call instead. This option is for 'reqwest' library only(default true)
  pub supportAsync: bool,
  /// If set, add support for reqwest-middleware. This option is for 'reqwest' library only(default false)
  pub supportMiddleware: bool,
  /// If set, return type wraps an enum of all possible 2xx schemas. This option is for 'reqwest' library only (default false)
  pub supportMultipleResponses: bool,
  /// Setting this property to true will generate functions with a single argument containing all API endpoint parameters instead of one argument per parameter.(default false)
  pub useSingleRequestParameter: bool,
  /// Whether to include AWS v4 signature support (default false)
  pub withAWSV4Signature: bool,
}
impl Default for OpenAPIRustGeneratorConfigs {
  fn default() -> Self {
    Self {
      bestFitInt: false,
      enumNameSuffix: Default::default(),
      hideGenerationTimestamp: true,
      library: "reqwest".to_string(),
      packageName: "openapi".to_string(),
      packageVersion: "1.0.0".to_string(),
      preferUnsignedInt: false,
      supportAsync: true,
      supportMiddleware: false,
      supportMultipleResponses: false,
      useSingleRequestParameter: false,
      withAWSV4Signature: false,
    }
  }
}
impl OpenAPIRustGeneratorConfigs {
  /// Instantiate
  pub fn new(cli: &Cli) -> Self {
    Self {
      packageName: cli.get_lib_name(),
      ..Default::default()
    }
  }
  /// Copy spec file if applicable
  pub async fn copy_spec_file(
    &self,
    cli: &Cli,
  ) -> Result<(), YAMLGenerationError> {
    if let Some(local_api_spec_filepath) = cli.inner_cli.local_api_spec_filepath_opt.as_ref() {
      let spec_file_name = cli.try_get_spec_file_name()?;
      let contents = fs::read(local_api_spec_filepath).await?;
      write(spec_file_name, contents, Some("Copy spec file")).await?;
      Ok(())
    } else {
      Ok(())
    }
  }
  /// Write configs to yaml file
  pub async fn write_to_yaml_file(
    &self,
    cli: &Cli,
  ) -> Result<(), YAMLGenerationError> {
    let output_dir = cli.get_output_project_dir();
    let output_file_name = MakefileEnv::OPEN_API_GENERATOR_CONFIG_FILE;
    let output_file_path = output_dir.join(output_file_name);
    write(
      output_file_path,
      serde_yaml::to_string(self)?,
      Some("OpenAPI rust generator configs"),
    )
    .await?;
    Ok(())
  }
}

/// Create a testing spec file in given directory
///
/// Returns the name of the spec created
pub async fn create_testing_spec_file(cli: &Cli) -> Result<(), YAMLGenerationError> {
  let petstore_yaml: &'static str = testing::PETSTORE_YAML;
  let output_file_path = cli
    .inner_cli
    .local_api_spec_filepath_opt
    .clone()
    .ok_or_else(|| {
      YAMLGenerationError::ParameterError(ParameterError::TestingYAMLSpecPathMissing)
    })?;
  write(
    &output_file_path,
    petstore_yaml,
    Some("Created source OpenAPI testing YAML"),
  )
  .await?;
  Ok(())
}
