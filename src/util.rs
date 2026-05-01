use std::ffi::CStr;
use std::io::Write;
use std::path::Path;
use std::{env, mem, ptr};

use anyhow::Result;
use tempfile::NamedTempFile;
use tracing::instrument;

use crate::{config, output};

pub fn exit_with_error(e: anyhow::Error) {
    e.downcast_ref::<clap::Error>()
        .map(|clap_err| {
            let _ = clap_err.print();
            std::process::exit(clap_err.exit_code())
        })
        .unwrap_or_else(|| {
            output::print_error(format!("{:#}", e));
            std::process::exit(libc::EXIT_FAILURE)
        });
}

#[instrument(skip_all, err, fields(path = %path.display()))]
pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let parent = path.parent().unwrap();
    fs_err::create_dir_all(parent)?;

    let mut tmp = NamedTempFile::new_in(parent)?;
    tmp.write_all(contents.as_bytes())?;
    tmp.as_file().sync_all()?;
    tmp.persist(path)?;

    Ok(())
}

#[instrument(ret)]
pub fn get_shell() -> Option<String> {
    let cfg_shell = config::get().shell.clone();
    cfg_shell.or_else(|| {
        if env::var("IN_NIX_SHELL").ok().is_none() {
            env::var("SHELL").ok()
        } else {
            get_shell_by_pwd()
        }
    })
}

fn get_shell_by_pwd() -> Option<String> {
    let mut passwd = unsafe { mem::zeroed::<libc::passwd>() };
    let mut buf = vec![0; 2048];
    let mut result = ptr::null_mut::<libc::passwd>();

    let uid = unsafe { libc::getuid() };

    loop {
        let r =
            unsafe { libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(), buf.len(), &mut result) };

        if r != libc::ERANGE {
            break;
        }

        let newsize = buf.len().checked_mul(2)?;
        buf.resize(newsize, 0);
    }

    if result.is_null() || result != &mut passwd {
        return None;
    }

    let shell_ptr = passwd.pw_shell;
    if shell_ptr.is_null() {
        return None;
    }

    unsafe { CStr::from_ptr(shell_ptr).to_str().ok().map(String::from) }
}
