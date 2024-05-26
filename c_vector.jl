struct CVector{T}
    ptr::Ptr{T}
    len::Csize_t
end
function Base.convert(::Type{Vector{T}}, cv::CVector{T}) where {T}
    return unsafe_wrap(Vector{T}, cv.ptr, cv.len)
end
function Base.convert(::Type{CVector{T}}, v::Vector{T}) where {T}
    # This needs to be 
    return CVector(pointer(v), length(v))
end

macro gc_preserve(args...)
    expr = esc(args[end])
    var_names = Symbol[]
    var_exprs = Expr[]
    for arg in args[begin:end-1]
        var_name = gensym()
        push!(var_names, var_name)
        push!(var_exprs, :($var_name = $arg))
    end
    return quote
        $(var_exprs...)
        GC.@preserve $(var_names...) $expr
    end
end
