pub trait Repository<T>
where
    T: Send + Sync,
{
    fn fetch_all(&self) -> Vec<&T>;
}
