require 'mkmf'

RbConfig::MAKEFILE_CONFIG['DLEXT'] = 'dll'
append_ldflags '-Wl,--export-all-symbols'
create_makefile 'libcruby_sys'
