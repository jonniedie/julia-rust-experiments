struct Option{T}
    is_valid::Bool # Int for memory compatibility with Rust
    value::T
    function Option{T}() where {T}
        return new{T}(false) # Leave value undef
    end
    function Option(value::T) where {T}
        return new{T}(true, value)
    end
end

const OptionSome = Option
const OptionNone = Option

some(thing) = Option(thing)
none_of_type(T::Type) = Option{T}()

is_valid(option::Option) = getfield(option, :is_valid)

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
