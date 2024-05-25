struct Option{T}
    is_valid::Int32 # Int32 for memory compatibility with Rust
    value::T
    function Option{T}() where {T}
        return new{T}(0) # Leave value undef
    end
    function Option{T}(value::T) where {T}
        return new{T}(1, value)
    end
end

is_valid(option::Option) = getfield(option, :is_valid) == 1

function Base.getproperty(option::Option, sym::Symbol)
    error(
        """
        Cannot access Option fields directly.
        To check validity, use `is_valid`
        To access the value, use `unwrap` or `unwrap_or`
        """
    )
end

function unwrap(option::Option)
    if is_valid(option)
        return getfield(option, :value)
    else
        error("Tried unwrapping invalid `Option`")
    end
end

function unwrap_or(option::Option{T}, default::T) where {T}
    return is_valid(option) ? unwrap(option) : default
end
