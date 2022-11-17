
pub trait Storable {
    fn stringify(&self) -> String;
    fn as_type(&self) -> String;
}