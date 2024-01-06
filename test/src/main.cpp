#include <iostream>

int main(int argc, char **argv) {
    std::cout << "Hello, World\n";
    return 0;
}
let filename = PathBuf::from(format!("{}/{}.{}", root.to_str().unwrap(), MAIN, &args.lang));
let mut file = OpenOptions::new()
	.write(true)
	.create(true)
	.open(filename)?;

match args.lang.as_str
