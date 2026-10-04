# void-settingsd

`org.freedesktop.hostname1`, `timedate1` and `locale1` for Void Linux (runit),
so GNOME/Cinnamon/KDE/COSMIC settings panels can change hostname, timezone,
NTP, RTC mode, locale and keyboard layout. One bus-activated daemon, idle-exits
after 60 s. Design: `docs/superpowers/specs/2026-10-04-settingsd-design.md`.

Files written: `/etc/hostname`, `/etc/machine-info`, `/etc/localtime`,
`/etc/adjtime`, `/etc/locale.conf`, `/etc/rc.conf` (KEYMAP, TIMEZONE if set),
`/etc/X11/xorg.conf.d/*keyboard.conf`. NTP is a runit service link under
`/var/service` (`ntp-services=` in `/etc/settingsd.conf` sets the order,
default `chronyd,ntpd,openntpd`).

Testing: `void-settingsd --bus session --root DIR --no-auth --persist` under
`dbus-run-session`; `--root`/`--no-auth` are refused on the system bus.
