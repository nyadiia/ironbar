function(id, ptr)
    local cr = cairo.Context(ptr)
    _G['draw_' .. id](cr)
end