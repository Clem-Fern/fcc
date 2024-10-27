use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use super::{error::ManifestError, validate::validate_path_buf_1vec};
use log::debug;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FilesFromPath {
    #[serde(default)]
    recurse: bool,
    #[serde(deserialize_with = "validate_path_buf_1vec")]
    paths: Vec<PathBuf>,
}

impl FilesFromPath {
    pub fn get_files<P: AsRef<Path>>(
        &self,
        manifest_path: P,
    ) -> Vec<Result<PathBuf, ManifestError>> {
        let parent = manifest_path.as_ref().parent().unwrap_or(Path::new("./"));
        debug!("{}", parent.display());

        let mut vec = vec![];

        for path in &self.paths {
            let path = parent.join(path);
            if path.is_dir() {
                if self.recurse {
                } else {
                    match read_dir(&path) {
                        Ok(dir) => {
                            let mut content = dir
                                .map(|res| res.map(|e| e.path()))
                                .map(|res| {
                                    res.map_err(|err| ManifestError::IO(path.to_path_buf(), err))
                                })
                                .collect::<Vec<_>>();
                            vec.append(&mut content);
                        }
                        Err(err) => {
                            vec.push(Err(ManifestError::IO(path.to_path_buf(), err)));
                        }
                    }
                    //     .map_err(|err| ManifestError::IO(path.to_path_buf(), err))
                    //     .map(|res| res.map(|e| e.path()))
                    //     .collect::<Result<Vec<_>, io::Error>>()
                    //     .map_err(|err| ManifestError::IO(path.to_path_buf(), err))?;
                    // debug!("{:?}", dir);
                    // vec.append(&mut dir);
                }
            } else {
                vec.push(Ok(path));
            }
        }

        vec
    }
}
