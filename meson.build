project(
  'gdgbatterysaver',
  'rust',
  version: '0.2.2',
  license: 'MIT',
  meson_version: '>= 0.59'
)

cargo = find_program('cargo', required: true)
rustc = find_program('rustc', required: true)

if get_option('debug')
  if get_option('enable-tray')
    command = [
      cargo, 'run', '--features=tray',
      '&&',
      'cp', meson.global_source_root() / 'target' / 'debug' / meson.project_name(), '@OUTPUT@'
    ]
  else
    command = [
      cargo, 'run',
      '&&',
      'cp', meson.global_source_root() / 'target' / 'debug' / meson.project_name(), '@OUTPUT@'
    ]
  endif
else
  if get_option('enable-tray')
    command = [
      cargo, 'build', '--features=tray',
      '--release',
      '&&',
      'cp', meson.global_source_root() / 'target' / 'release' / meson.project_name(), '@OUTPUT@'
    ]
  else
    command = [
      cargo, 'build',
      '--release',
      '&&',
      'cp', meson.global_source_root() / 'target' / 'release' / meson.project_name(), '@OUTPUT@'
    ]
  endif
endif

prefix = get_option('prefix')

datadir = prefix / get_option('datadir')
icondir = datadir / 'pixmaps'
desktopdir = datadir / 'applications'
bindir = prefix / get_option('bindir')
libdir = prefix / get_option('libdir')
helpdir = libdir / 'gdgbatterysaver' 

global_conf = configuration_data()
global_conf.set_quoted('HELPER_PATH', helpdir / 'gdghelper' )

config = configure_file(
  input: 'src/config.rs.in',
  output: 'config.rs',
  configuration: global_conf
)

run_command(
  'cp',
  meson.project_build_root() / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)


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

install_data('misc/gdghelper',
  install_dir: helpdir
)

install_data('misc/gdgbatterysaver.desktop',
  install_dir: desktopdir
)
