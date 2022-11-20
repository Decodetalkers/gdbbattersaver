project(
  'gdgbatterysaver',
  'rust',
  version: '0.1',
  license: 'MIT-2.0',
  meson_version: '>= 0.59'
)

cargo = find_program('cargo', required: true)
rustc = find_program('rustc', required: true)

if get_option('debug')
  command = [
    cargo, 'build',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'debug' / meson.project_name(), '@OUTPUT@'
  ]
else
  command = [
    cargo, 'build',
    '--release',
    '&&',
    'cp', meson.global_source_root() / 'target' / 'release' / meson.project_name(), '@OUTPUT@'
  ]
endif

prefix = get_option('prefix')

datadir = prefix / get_option('datadir')
icondir = datadir / 'pixmaps'
desktopdir = datadir / 'applications'
bindir = prefix / get_option('bindir')

gdgbattersaver_target = custom_target('gdgbatterysaver',
  output: 'gdgbatterysaver',
  build_by_default: true,
  install: true,
  install_dir: bindir,
  console: true,
  command: command
)

install_data('misc/icons/Green_Dam_Girl.png',
  install_dir: icondir
)
install_data('misc/gdgbatterysaver.desktop',
  install_dir: desktopdir
)
