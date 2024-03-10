local lgi = require('lgi')
cairo = lgi.cairo

-- fix to allow creating context from pointer
cairo.Context.create = nil