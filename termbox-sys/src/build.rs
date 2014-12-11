use std::os;
use std::io::Command;
use std::io::process::InheritFd;

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());

    configure();
    build();
    install(&dst);
    println!("cargo:rustc-flags=-L {} -l termbox:static", dst.join("lib").display());
}

fn configure() {
    let mut cmd = waf();
    cmd.arg("configure");
    cmd.arg("--prefix=/");

    let target = os::getenv("TARGET").unwrap();
    let mut cflags;
    if target.as_slice().contains("i686") {
        cflags = "-m32"
    } else if target.as_slice().contains("x86_64") {
        cflags = "-m64 -fPIC"
    } else {
        cflags = ""
    }
    println!("waf configure: setting CFLAGS to: `{}`", cflags);
    os::setenv("CFLAGS", cflags);

    run(&mut cmd);
    os::unsetenv("CFLAGS");
}

fn build() {
    let mut cmd = waf();
    cmd.arg("build");
    cmd.arg("--targets=termbox_static");
    run(&mut cmd);
}

fn install(dst: &Path) {
    let mut cmd = waf();
    cmd.arg("install");
    cmd.arg("--targets=termbox_static");
    cmd.arg(format!("--destdir={}", dst.display()));
    run(&mut cmd);
}

fn waf() -> Command {
    let cargo_dir = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let termbox_dir = cargo_dir.join("termbox");
    let mut cmd = Command::new("./waf");
    cmd.cwd(&termbox_dir);
    cmd
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
                .stderr(InheritFd(2))
                .status()
                .unwrap()
                .success());
}
