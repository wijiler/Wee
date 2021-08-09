use std::io;
use std::fs::File;
use std::io::Write;


fn createfile() -> io::Result<()> {
    let mut filename = String::new();
    println!("Enter file name here :");
    let _filenameoutput = io::stdin().read_line(&mut filename).unwrap();
    let mut file = File::create(filename)?;
    println!("Wee - the whatever editor\n");
    let mut write = String::new();
    let _writing = io::stdin().read_line(&mut write).unwrap();
    write!(file, "{}", write)?;
    Ok(())
}

fn main() -> io::Result<()>{
    
        createfile();
    Ok(())
}
