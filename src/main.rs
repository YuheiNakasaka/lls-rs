use clap::Parser;
use nc::getdents64;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    #[arg(short, long, default_value_t = 5 * 1024 * 1024)]
    buf_size: usize,
}

#[derive(Debug, PartialEq)]
enum ExitCode {
    Success = 0,
    Error = 1,
}

fn run(path: PathBuf, buf_size: usize) -> ExitCode {
    if !path.is_dir() {
        println!("{:?} is not a directory", path);
        return ExitCode::Error;
    }

    let ret = unsafe { nc::openat(nc::AT_FDCWD, path, nc::O_DIRECTORY, 0) };
    let fd = ret.unwrap();

    let mut buf: Vec<u8> = vec![0; buf_size];
    let mut out: Vec<String> = vec![];

    loop {
        let ret = unsafe { getdents64(fd, buf.as_mut_ptr() as usize, buf_size) };
        let n: usize = ret.unwrap() as usize;

        if n == 0 {
            break;
        }

        let mut bufp: usize = 0;
        while bufp < n {
            let dirent = unsafe { &*(buf.as_ptr().add(bufp) as *const nc::linux_dirent64_t) };
            bufp += dirent.d_reclen as usize;

            if dirent.d_ino == 0 {
                continue;
            }

            let mut name_vec: Vec<u8> = vec![];
            for i in 0..nc::PATH_MAX {
                let c = dirent.d_name[i as usize];
                if c == 0 {
                    break;
                }
                name_vec.push(c);
            }

            let _name = String::from_utf8(name_vec).unwrap();
            if _name == "." || _name == ".." {
                continue;
            }

            out.push(_name);
        }
    }

    println!("{}", out.join("\n"));
    ExitCode::Success
}

fn main() {
    let args = Cli::parse();
    run(args.path, args.buf_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let path = PathBuf::from("./test_files");
        let buf_size = 5 * 1024 * 1024;
        assert_eq!(run(path, buf_size), ExitCode::Success);
    }
}
