# void-settingsd

`org.freedesktop.hostname1`, `timedate1` and `locale1` D-Bus services for Void
Linux (runit), so GNOME, Cinnamon, KDE and COSMIC settings panels can change
the hostname, timezone, NTP, RTC mode, locale and keyboard layout without
systemd. One bus-activated daemon that exits after 60 s idle, plus
`timedatectl`, `hostnamectl` and `localectl` wrappers.

## Install

From the voidlab binary repository (x86_64 glibc):

```sh
printf 'repository=https://github.com/mendescotta/voidlab/releases/download/repo\nbestmatching=true\n' |
    sudo tee /etc/xbps.d/20-voidlab.conf
sudo xbps-install -S void-settingsd    # accept the voidlab signing key
```

From source (needs `rust`, `cargo`; runtime `dbus`, `polkit`, `tzdata`):

```sh
cargo build --release
sudo install -Dm755 target/release/void-settingsd /usr/libexec/void-settingsd
sudo install -Dm755 target/release/void-settingsctl /usr/bin/void-settingsctl
for t in timedatectl hostnamectl localectl; do sudo ln -sf void-settingsctl /usr/bin/$t; done
sudo install -Dm644 data/org.freedesktop.settingsd.conf /usr/share/dbus-1/system.d/org.freedesktop.settingsd.conf
sudo install -Dm644 data/org.freedesktop.settingsd.policy /usr/share/polkit-1/actions/org.freedesktop.settingsd.policy
for n in hostname1 timedate1 locale1; do
    sudo install -Dm644 data/org.freedesktop.$n.service /usr/share/dbus-1/system-services/org.freedesktop.$n.service
done
```

Nothing to enable: the system bus starts the daemon when a settings panel (or
`timedatectl` etc.) first talks to it. Requires a running `dbus` and `polkit`.

## What it writes

`/etc/hostname`, `/etc/machine-info`, `/etc/localtime`, `/etc/adjtime`,
`/etc/locale.conf`, `/etc/rc.conf` (KEYMAP, TIMEZONE if set) and
`/etc/X11/xorg.conf.d/*keyboard.conf`. NTP is a runit service link under
`/var/service`; `ntp-services=` in `/etc/settingsd.conf` sets the order
(default `chronyd,ntpd,openntpd`).

## timedatectl / hostnamectl / localectl

`void-settingsctl` is a multi-call client; run it through the symlinks above or
as `void-settingsctl <tool> ...`.

| Tool | Commands |
|---|---|
| `timedatectl` | status, show, set-timezone, list-timezones, set-ntp, set-local-rtc |
| `hostnamectl` | status, hostname, set-hostname (`--static/--pretty/--transient`), set-icon-name, set-chassis, set-deployment, set-location |
| `localectl` | status, list-locales, set-locale, set-keymap, set-x11-keymap |

These are not the full systemd CLIs (no `set-time`).

## Development

```sh
cargo test
dbus-run-session -- target/debug/void-settingsd --bus session --root DIR --no-auth --persist
```

`--root` and `--no-auth` are refused on the system bus.

## License

GPL-3.0-or-later.
