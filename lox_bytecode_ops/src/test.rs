use crate::{codec::Encode, writer::OpWriter, Operation, StringSymbol};

#[test]
fn encode_operations() {
    let operations = [
        Operation::LoadNumber(1.),
        Operation::LoadBool(true),
        Operation::LoadString(StringSymbol(1)),
        Operation::LoadNil,
        Operation::LoadNil,
        Operation::Negative,
        Operation::Not,
        Operation::Plus,
        Operation::Minus,
        Operation::Multiply,
        Operation::Divide,
        Operation::And,
        Operation::Or,
        Operation::Greater,
        Operation::GreaterEqual,
        Operation::Less,
        Operation::LessEqual,
        Operation::Equal,
        Operation::NotEqual,
    ];

    let mut writer = OpWriter::new();
    operations.as_slice().encode(&mut writer);
    assert_eq!(
        writer.flush(),
        [0].into_iter()
            .chain(1f64.to_le_bytes())
            .chain([2, 1])
            .chain([1])
            .chain(1u32.to_le_bytes())
            .chain([3, 3])
            .chain(4..=17)
            .collect::<Vec<u8>>()
    );
}
