use std::fs;
use std::env;
use std::path;
fn main()
{
    // match env::var("PATH")
    // {
    // 	Ok(path) => println!("Path: {}", path),
    //     Err(e) => println!("Couldn't read PATH ({})", e),
    // };
    match env::current_dir()
    {
    	Ok(directory) =>  list_directory(&directory, false),
    	Err(e) => println!("Couldn't get current directory: ({})", e),
    };
}

fn list_directory(init_path: &path::Path, is_recursive: bool)
{
	let file_paths = fs::read_dir(init_path).unwrap();
	for path in file_paths
	{
		match path
		{
			Ok(p) =>
			{
				if p.metadata().unwrap().is_dir() && is_recursive
				{
					// println!("This is a directory!", );
					list_directory(&p.path(), is_recursive);
				}
				else
				{
					// println!("This is a file!");
					match p.path().to_str()
					{
						Some(cp) =>
						{
							println!("{}", cp);
						}
						None =>
						{
							panic!("Something horrible went wrong!");
						}
					}
				}
			}
			Err(e) =>
			{
				println!("Error encountered: {}", e);
			}
		}
	}
}
