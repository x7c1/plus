mod params;

pub use params::Params;

use crate::core::support::release::CargoToml;
use crate::core::support::{get_artifacts_dir, get_tar_path, program_exists};
use crate::TaskResult;
use shellwork::core::command;
use std::path::PathBuf;

pub struct Task;

impl Task {
    pub fn start(&self, params: &Params) -> TaskResult<()> {
        println!("[start] #params to package...{:#?}", params);
        params
            .files
            .filter_cargo_tomls(&params.target_packages)
            .try_for_each(|toml| self.compress(params, &toml?))
    }

    fn compress(&self, params: &Params, toml: &CargoToml) -> TaskResult<()> {
        println!("[start] #params to compress...{:#?}", toml);

        let params = ParamsToCompress::from_cargo_toml(params, toml);
        let runner = command::program("tar")
            .arg("--xz")
            .args(&["--create", "--file", &params.dst_path.to_string_lossy()])
            .args(&["--directory", &params.src_dir_path.to_string_lossy()])
            .arg(&params.src_file_name);

        runner.prepare(program_exists)?.spawn()?;
        Ok(())
    }
}

struct ParamsToCompress {
    src_dir_path: PathBuf,
    src_file_name: String,
    dst_path: PathBuf,
}

impl ParamsToCompress {
    fn from_cargo_toml(params: &Params, toml: &CargoToml) -> ParamsToCompress {
        ParamsToCompress {
            src_dir_path: get_artifacts_dir(params.target, params.build_mode),
            src_file_name: toml.settings.package_name.as_str().to_string(),
            dst_path: get_tar_path(params.target, params.build_mode, &toml.contents.package),
        }
    }
}
