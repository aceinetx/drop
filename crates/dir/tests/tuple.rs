use dir::IR;

#[test]
#[should_panic]
fn test_tuple_no_void() {
    let mut ir = IR::default();
    ir.create_tuple(vec![ir.get_type_u8(), ir.get_type_u0()], false);
}
