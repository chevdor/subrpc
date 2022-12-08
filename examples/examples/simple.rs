use subrpc_core::*;

fn main() {
	// Initialize the logger
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("none")).init();

	// Get the chain name from the args
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		panic!("Pass the name of a chain as argument");
	}
	let chain = &args[1];

	// We use a temp file to ensure we don't touch your production local data
	let local_data_file = std::path::Path::new("/tmp/subrpc_demo.json");
	println!("Using local data from: {}", local_data_file.display());

	// We use the builder pattern to initialize our local data with one registry
	const REGISTRY_URL: &str = "https://gist.githubusercontent.com/chevdor/a8b381911c28f6de02dde62ed1a17dec/raw/b6fea4c0a688fafe0514281cf68eec95c23f3c49/data2.json";
	let db = LocalData::init(local_data_file, false)
		.expect("Should be able to load local data")
		.load()
		.expect("Should load properly")
		.add_registry(Registry::new("Simple Example", REGISTRY_URL))
		.refresh();

	// This shows the list of registries we are aware of and how many RPC endpoints they contain
	db.print_summary();

	// Search for endpoins for the chain we passed as arg
	let endpoints = db.get_endpoints(Some(&chain));

	// Show result to the user
	if endpoints.is_empty() {
		println!("No RPC Endpoints found for chain '{chain}' in your registries.");
	} else {
		println!("RPC Endpoints for {chain}:");
		endpoints.iter().for_each(|e| {
			println!("- {:<20}: {}", e.name, e.url);
		})
	}
}
