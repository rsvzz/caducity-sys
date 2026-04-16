inc_src = include_directories('../src')

gtk_dep = dependency('gtk4', required : true, version: '>= 4.14')
adw_dep = dependency('libadwaita-1', required : true, version: '>=1.5')
sqlite_dep = dependency('sqlite3', required: true, version: '>=3.46')

cargo = find_program('cargo', required: true)

#install_data('../data/styles/io.github.rsvzz.csven.css', install_dir: get_option('prefix') / 'share/csven/styles')
#install_data('../data/metainfo/io.github.rsvzz.csven.metainfo.xml', install_dir: get_option('prefix') / 'share/metainfo')
#nstall_data('../data/icons/io.github.rsvzz.csven.svg', install_dir: get_option('prefix') / 'share/icons/hicolor/scalable/apps')
#nstall_data('../data/desktop/io.github.rsvzz.csven.desktop', install_dir: get_option('prefix') /'share/applications')

icons_file = files(
   '../data/ui/main.ui',
   '../data/ui/product.ui',
   '../data/ui/category.ui',
   '../data/ui/inventory.ui',
   '../data/ui/home.ui'
)
install_data(icons_file, install_dir: get_option('prefix') / 'share/caducity-sys/ui')

buildtype = get_option('buildtype')
cargo_flag = []
src_exe = 'debug'

if buildtype != 'debug'
  cargo_flag = ['--release']
  src_exe = 'release'
endif

cargo_build = custom_target('cargo-build',
  output: 'caducity-build',
  build_by_default: true,
  env:['CARGO_HOME=' + meson.project_build_root() / 'cargo'],
  command: [
    'cargo', 'build'] + cargo_flag + [
    '--manifest-path', meson.project_source_root() / 'Cargo.toml',
    '--target-dir', meson.project_build_root() / 'src'
  ],
  build_always_stale: true
)

exe_name = 'caducity'
if host_machine.system() == 'windows'
  exe_name += '.exe'
endif

custom_target('caducity-sys',
  output: exe_name,
  build_by_default: true,
  build_always_stale: true,
  depends: cargo_build,
  env:['CARGO_HOME=' + meson.project_build_root() / 'cargo'],
  install: true,
  install_dir : get_option('bindir'),
  command: ['cp', meson.project_build_root() / 'src/' + src_exe + '/' + exe_name, '@OUTPUT@']
)
