using SumTypes

@sum_type Option{T} begin
    OSome{T}(::T)
    ONone{T}()
end
struct COption{T}
    is_valid::Bool
    value::T
end
function Base.convert(::Type{Option{T1}}, c_option::COption{T2}) where {T1, T2}
    return c_option.is_valid ? OSome(convert(T1, c_option.value)) : ONone{T1}()
end
function Base.convert(::Type{COption{T1}}, option::Option{T2}) where {T1, T2}
    return @cases option begin
        OSome(value) => COption(true, convert(T1, value))
        ONone => COption(false, T1()) # Just hope there's an empty constructor method
    end
end

function unwrap(option::Option)
    return @cases option begin
        OSome(value) => value
        ONone => error("Tried unwrapping invalid Option")
    end
end
function unwrap(coption::COption{T}) where {T}
    return unwrap(convert(Option{T}, coption))
end

function unwrap_or(option::Option{T}, default::T) where {T}
    return @cases option begin
        OSome(value) => value
        ONone => default
    end
end
function unwrap_or(coption::COption{T}, default) where {T}
    return unwrap_or(convert(Option{T}, coption), default)
end
