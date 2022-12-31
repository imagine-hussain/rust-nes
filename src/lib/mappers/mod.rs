mod mapper; mod mapper_000;

pub use mapper::Mapper;
pub use mapper_000::Mapper000;

use crate::cartridge::Header;

// TODO: use a dynamic dispatch here with a boxed trait object
pub fn select_mapper(mapper_id: u8, header: &Header) -> Mapper000  {
    match mapper_id {
        0 => Mapper000::new(header),
        _ => unimplemented!("Mapper not implemented: {}", mapper_id),
    }
}
