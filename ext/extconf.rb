require 'mkmf'

append_ldflags '-Wl,--export-all-symbols'
create_makefile 'libcruby_sys'
