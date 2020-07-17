mod metainfo;
mod tracker;

pub use metainfo::*;
pub use tracker::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
