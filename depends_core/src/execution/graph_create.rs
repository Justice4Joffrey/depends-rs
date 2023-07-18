/// Convenience so that types which derive `Graph` can be easily accessed.
pub trait GraphCreate {
    type Graph<R>;
}
