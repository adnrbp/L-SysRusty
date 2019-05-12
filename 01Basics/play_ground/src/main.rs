mod in_array;
mod own_borrow;
mod struct_meth;


fn main() {
    //ArrayInspection
    in_array::array_inspection();

    in_array::string_inspection();

    //validate ownership
    own_borrow::ownership_validation();

    own_borrow::copy_values();

    own_borrow::borrowing_values();

    own_borrow::vector_counter();

    //Struct usage
    struct_meth::area_finder();
}
