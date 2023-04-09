use crate::pattern::component::component::Component;
use crate::pattern::component::file::File;
use crate::pattern::component::fold::Folder;

mod file;
mod fold;
mod component;

#[test]
fn test_component() {
    let file1 = File::new("File 1");
    let file2 = File::new("File 2");
    let file3 = File::new("File 3");

    let mut folder1 = Folder::new("Folder 1");
    folder1.add_component(file1);

    let mut folder2 = Folder::new("Folder 2");
    folder2.add_component(file2);
    folder2.add_component(file3);
    folder2.add_component(folder1);

    //      folder2
    //      /   |    \
    //   file2 file3 folder1
    //                |
    //                file1


    folder2.search("rust");

}