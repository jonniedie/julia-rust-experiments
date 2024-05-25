using Libdl

include("option.jl")

dlsym("example", :airport_security_check)

struct Book
    title::String
end

struct Laptop end

@kwdef struct Backpack
    laptop::Option{Laptop} = Option{Laptop}()
    books::Vector{Book}
end

function main()
    backpack = Backpack(
        laptop = Laptop(),
        books = [
            Book("Rust Atomics and Locks"),
            Book("Hands-on Design Patterns and Best Practices with Julia"),
        ]
    )

    bin = []
    while true
        passed_security = @ccall "libexample".airport_security_check(backpack::Backpack)::Bool
        if passed_security
            laptop = pop!(bin)
            @ccall "libexample".insert_laptop(backpack::Backpack, laptop::Laptop)::Nothing
            break
        else
            laptop = @ccall "libexample".remove_laptop(backpack::Backpack)::Laptop
            push!(bin, laptop)
        end
    end

    @ccall "libexample".get_some_work_done(backpack.laptop::Option{Laptop})::Nothing
end