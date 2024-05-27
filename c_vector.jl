struct CVector{T}
    ptr::Ptr{T}
    len::Csize_t
end
function Base.convert(::Type{Vector{T}}, cv::CVector{T}) where {T}
    return unsafe_wrap(Vector{T}, cv.ptr, cv.len)
end
function Base.convert(::Type{CVector{T}}, v::Vector{T}) where {T}
    # This needs to be 
    return CVector(pointer(v), Csize_t(length(v)))
end
