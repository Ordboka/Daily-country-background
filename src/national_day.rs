use crate::date::Date;

#[derive(Debug)]
pub struct NationalDay {
    pub country: String,
    pub date: Date,
    pub extra_info: String,
}
