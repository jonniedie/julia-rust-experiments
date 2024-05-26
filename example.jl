include("option2.jl")

@kwdef struct Book
    title::String = ""
end
struct CBook
    title::Cstring
end

Base.convert(::Type{Book}, cbook::CBook) = Book(unsafe_string(cbook.title))
Base.convert(::Type{CBook}, book::Book) = CBook(convert(Cstring, pointer(book.title)))

@kwdef struct Laptop
    weight_kg::Float64 = 0.0
end
# This one's fine because Cdouble === Float64
const CLaptop = Laptop

@kwdef struct Backpack
    laptop::Option{Laptop} = ONone{Laptop}()
    # books::Vector{Book} = Book[]
end
@kwdef struct CBackpack
    # has_laptop::Bool
    # raw_laptop::CLaptop
    laptop::COption{CLaptop}
    # ptr_books::Ptr{Book}
    # num_books::Csize_t
end

function Base.convert(::Type{Backpack}, cbackpack::CBackpack)
    return Backpack(
        laptop = cbackpack.laptop.is_valid ? OSome(cbackpack.laptop.value) : ONone{Laptop}(),
        # books = unsafe_wrap(Vector{Book}, cbackpack.ptr_books, cbackpack.num_books)
    )
end
function Base.convert(::Type{CBackpack}, backpack::Backpack)
    return CBackpack(
        # has_laptop = backpack.is_valid,
        # raw_laptop = unwrap_or(backpack.laptop, CLaptop()),
        laptop = convert(COption{CLaptop}, backpack.laptop),
        # ptr_books = pointer(backpack.books),
        # num_books = length(backpack.books),
    )
end

function main()
    laptop = OSome(Laptop(1.0))
    # books = [
    #     Book("Rust Atomics and Locks"),
    #     Book("Hands-on Design Patterns and Best Practices with Julia"),
    # ]
    backpack = Backpack(
        laptop = laptop,
        # books = books,
    )
    cbackpack = Ref(convert(CBackpack, backpack))

    # GC.@preserve books begin
        bin = Laptop[]
        while true
            passed_security = @ccall "libexample".airport_security_check(cbackpack[]::CBackpack)::Bool
            if passed_security
                laptop = pop!(bin)
                @ccall "libexample".insert_laptop(cbackpack::Ref{CBackpack}, laptop::Laptop)::Nothing
                break
            else
                # filter!(!contains("atomic"), backpack.books)
                # maybe_book = @ccall "libexample".remove_book_named("Rust Atomics and Locks"::Cstring)::Option{Book}
                maybe_laptop = @ccall "libexample".remove_laptop(cbackpack::Ref{CBackpack})::COption{Laptop}
                push!(bin, unwrap(maybe_laptop))
            end
        end
    # end
    backpack = convert(Backpack, cbackpack[])
end 