require 'mkmf'

append_cflags '-Wno-attributes'
append_ldflags '-Wl,--export-all-symbols'
create_makefile 'libcruby_sys'
