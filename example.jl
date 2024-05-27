include("option.jl")
include("c_vector.jl")

# This one's fine because Cdouble === Float64
@kwdef struct Laptop
    mass_kg::Float64 = 0.0
end

@kwdef struct Pen
    mass_kg::Float64 = 0.0
end

@kwdef struct Backpack
    laptop::Option{Laptop} = ONone{Laptop}()
    # pens::Vector{Pen} = Pen[]
end
@kwdef mutable struct CBackpack
    laptop::COption{Laptop}
    # pens::CVector{Pen}
end

function Base.convert(::Type{Backpack}, cbackpack::CBackpack)
    return Backpack(
        laptop = convert(Option{Laptop}, cbackpack.laptop),
        # pens = convert(Vector{Pen}, cbackpack.pens),
    )
end
function Base.convert(::Type{CBackpack}, backpack::Backpack)
    return CBackpack(
        laptop = convert(COption{Laptop}, backpack.laptop),
        # pens = convert(CVector{Pen}, backpack.pens),
    )
end
function Base.cconvert(::Type{Ptr{CBackpack}}, cbackpack::CBackpack)
    return pointer_from_objref(cbackpack)
end

function main()
    # pens = [
    #     Pen(0.01),
    #     Pen(0.015),
    # ]
    backpack = Backpack(
        laptop = OSome(Laptop(1.0)),
        # pens = pens,
    )
    cbackpack = convert(CBackpack, backpack)

    # GC.@preserve pens begin
        bin = Laptop[]
        while true
            passed_security = @ccall "libexample".airport_security_check(cbackpack::CBackpack)::Bool
            if passed_security
                laptop = pop!(bin)
                @ccall "libexample".insert_laptop(cbackpack::Ptr{CBackpack}, laptop::Laptop)::Nothing
                break
            else
                maybe_laptop = @ccall "libexample".remove_laptop(cbackpack::Ptr{CBackpack})::COption{Laptop}
                push!(bin, unwrap(maybe_laptop))
            end
        end
        backpack = convert(Backpack, cbackpack)
    # end
end 