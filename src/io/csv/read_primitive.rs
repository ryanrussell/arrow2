use csv::StringRecord;
use lexical_core;

use crate::{array::Primitive, datatypes::*};
use crate::{array::PrimitiveArray, buffer::NativeType};

pub trait PrimitiveParser<T: NativeType + lexical_core::FromLexical, E> {
    fn parse(&self, string: &str, _: &DataType, _: usize) -> Result<Option<T>, E> {
        // default behavior is infalible: `None` if unable to parse
        Ok(lexical_core::parse(string.as_bytes()).ok())
    }
}

pub fn new_primitive_array<
    T: NativeType + lexical_core::FromLexical,
    E,
    P: PrimitiveParser<T, E>,
>(
    line_number: usize,
    rows: &[StringRecord],
    col_idx: usize,
    data_type: &DataType,
    parser: &P,
) -> Result<PrimitiveArray<T>, E> {
    let iter = rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| match row.get(col_idx) {
            Some(s) => {
                if s.is_empty() {
                    return Ok(None);
                }
                parser.parse(s, &data_type, line_number + row_index)
            }
            None => Ok(None),
        });
    // Soundness: slice is trusted len.
    Ok(unsafe { Primitive::<T>::try_from_trusted_len_iter(iter) }?.to(data_type.clone()))
}