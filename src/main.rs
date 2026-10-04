mod auth;
mod files;
mod hostname;
mod locale;
mod timedate;

use auth::Ctx;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

const IDLE_SECS: u64 = 60;

fn main() -> zbus::Result<()> {
    let mut root = PathBuf::from("/");
    let mut session = false;
    let mut no_auth = false;
    let mut persist = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--root" => root = PathBuf::from(args.next().expect("--root needs a path")),
            "--bus" => session = args.next().as_deref() == Some("session"),
            "--no-auth" => no_auth = true,
            "--persist" => persist = true,
            "-h" | "--help" => {
                println!("void-settingsd [--root DIR] [--bus system|session] [--no-auth] [--persist]");
                return Ok(());
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    let test_mode = root != PathBuf::from("/");
    if (test_mode || no_auth) && !session {
        eprintln!("--root/--no-auth are only allowed with --bus session");
        std::process::exit(2);
    }
    let ctx = Arc::new(Ctx {
        root,
        no_auth,
        test_mode,
        last_activity: AtomicU64::new(0),
    });
    ctx.touch();

    async_io::block_on(async {
        let b = if session {
            zbus::connection::Builder::session()?
        } else {
            zbus::connection::Builder::system()?
        };
        let _conn = b
            .name("org.freedesktop.hostname1")?
            .name("org.freedesktop.timedate1")?
            .name("org.freedesktop.locale1")?
            .serve_at("/org/freedesktop/hostname1", hostname::Hostname(ctx.clone()))?
            .serve_at("/org/freedesktop/timedate1", timedate::Timedate(ctx.clone()))?
            .serve_at("/org/freedesktop/locale1", locale::Locale(ctx.clone()))?
            .build()
            .await?;
        loop {
            async_io::Timer::after(Duration::from_secs(5)).await;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            if !persist && now.saturating_sub(ctx.last_activity.load(Ordering::Relaxed)) > IDLE_SECS {
                return Ok(());
            }
        }
    })
}
