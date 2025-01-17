use file_system::file::File;

pub struct DiskManager<'a> {
    file: &'a File,
    pages: usize,
}

impl<'a> DiskManager<'a> {
    fn new(file: &'a File) -> Self {
        Self {
            file,
            pages: 0usize,
        }
    }
}
