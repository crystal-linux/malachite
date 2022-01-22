use crate::internal::structs::Config;

mod read;

pub fn read_cfg() -> Config {
    read::read_cfg()
}
