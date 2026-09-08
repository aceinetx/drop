use dir::IR;

#[test]
fn test_ptr_type_reuse() {
    let mut ir = IR::default();
    assert_eq!(
        ir.create_ptr(ir.get_type_u0()),
        ir.create_ptr(ir.get_type_u0())
    );
    assert_eq!(
        ir.create_ptr(ir.get_type_u8()),
        ir.create_ptr(ir.get_type_u8())
    );
    assert_eq!(
        ir.create_ptr(ir.get_type_u0()),
        ir.create_ptr(ir.get_type_u0())
    );

    assert_ne!(
        ir.create_ptr(ir.get_type_u0()),
        ir.create_ptr(ir.get_type_u8())
    );
}

#[test]
fn test_const_type_reuse() {
    let mut ir = IR::default();
    assert_eq!(
        ir.create_const(ir.get_type_u0()),
        ir.create_const(ir.get_type_u0())
    );
    assert_eq!(
        ir.create_const(ir.get_type_u8()),
        ir.create_const(ir.get_type_u8())
    );
    assert_eq!(
        ir.create_const(ir.get_type_u0()),
        ir.create_const(ir.get_type_u0())
    );

    assert_ne!(
        ir.create_const(ir.get_type_u0()),
        ir.create_const(ir.get_type_u8())
    );
}

#[test]
fn test_tuple_type_reuse() {
    let mut ir = IR::default();
    let tuple_1 = ir.create_tuple(vec![ir.get_type_u8(), ir.get_type_u8()]);
    let tuple_2 = ir.create_tuple(vec![ir.get_type_u8(), ir.get_type_u8()]);
    let tuple_3 = ir.create_tuple(vec![ir.get_type_u8(), ir.get_type_u8()]);

    assert_eq!(tuple_1, tuple_2);
    assert_eq!(tuple_2, tuple_3);

    let tuple_3 = ir.create_tuple(vec![ir.get_type_u8(), ir.get_type_u16()]);
    assert_ne!(tuple_1, tuple_3);
}
