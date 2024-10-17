use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Utc};
use ldk_node::config::Config;
use ldk_node::graph::NetworkGraph;
use ldk_node::{Builder, LogLevel};

use ldk_node::lightning::ln::msgs::SocketAddress;

use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;

const PERSIST_PATH: &str = "/home/tnull/html_bolt12.support/index.html";

fn main() {
	let mut config = Config::default();
	config.network = Network::Bitcoin;

	let mut builder = Builder::from_config(config);
	builder.set_storage_dir_path("/home/tnull/ldk_node_om_support_storage/".to_string());
	builder.set_log_level(LogLevel::Debug);

	let node = Arc::new(builder.build().unwrap());
	node.start().unwrap();

	let event_node = Arc::clone(&node);
	std::thread::spawn(move || loop {
		event_node.wait_next_event();
		event_node.event_handled();
	});

	println!("Node ID: {}", node.node_id());

	let nodes_to_connect: Vec<(PublicKey, SocketAddress)> = vec![
		(
			"03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".parse().unwrap(),
			"3.33.236.230:9735".parse().unwrap(),
		),
		(
			"0217890e3aad8d35bc054f43acc00084b25229ecff0ab68debd82883ad65ee8266".parse().unwrap(),
			"23.237.77.11:9735".parse().unwrap(),
		),
		(
			"035e4ff418fc8b5554c5d9eea66396c227bd429a3251c8cbc711002ba215bfc226".parse().unwrap(),
			"170.75.163.209:9735".parse().unwrap(),
		),
		(
			"02f1a8c87607f415c8f22c00593002775941dea48869ce23096af27b0cfdcc0b69".parse().unwrap(),
			"52.13.118.208:9735".parse().unwrap(),
		),
		(
			"0364913d18a19c671bb36dd04d6ad5be0fe8f2894314c36a9db3f03c2d414907e1".parse().unwrap(),
			"192.243.215.102:9735".parse().unwrap(),
		),
		(
			"02cfdc6b60e5931d174a342b20b50d6a2a17c6e4ef8e077ea54069a3541ad50eb0".parse().unwrap(),
			"52.89.237.109:9735".parse().unwrap(),
		),
		(
			"024bfaf0cabe7f874fd33ebf7c6f4e5385971fc504ef3f492432e9e3ec77e1b5cf".parse().unwrap(),
			"52.1.72.207:9735".parse().unwrap(),
		),
		(
			"037659a0ac8eb3b8d0a720114efc861d3a940382dcfa1403746b4f8f6b2e8810ba".parse().unwrap(),
			"34.78.139.195:9735".parse().unwrap(),
		),
		(
			"03cde60a6323f7122d5178255766e38114b4722ede08f7c9e0c5df9b912cc201d6".parse().unwrap(),
			"34.65.85.39:9745".parse().unwrap(),
		),
		(
			"0294ac3e099def03c12a37e30fe5364b1223fd60069869142ef96580c8439c2e0a".parse().unwrap(),
			"8.210.134.135:26658".parse().unwrap(),
		),
		(
			"03037dc08e9ac63b82581f79b662a4d0ceca8a8ca162b1af3551595b8f2d97b70a".parse().unwrap(),
			"34.68.41.206:9735".parse().unwrap(),
		),
		(
			"026165850492521f4ac8abd9bd8088123446d126f648ca35e60f88177dc149ceb2".parse().unwrap(),
			"45.86.229.190:9735".parse().unwrap(),
		),
		(
			"03c2abfa93eacec04721c019644584424aab2ba4dff3ac9bdab4e9c97007491dda".parse().unwrap(),
			"13.113.39.53:9735".parse().unwrap(),
		),
		(
			"030c3f19d742ca294a55c00376b3b355c3c90d61c6b6b39554dbc7ac19b141c14f".parse().unwrap(),
			"54.77.250.40:9735".parse().unwrap(),
		),
		(
			"034ea80f8b148c750463546bd999bf7321a0e6dfc60aaf84bd0400a2e8d376c0d5".parse().unwrap(),
			"213.174.156.66:9735".parse().unwrap(),
		),
		(
			"02e4971e61a3f55718ae31e2eed19aaf2e32caf3eb5ef5ff03e01aa3ada8907e78".parse().unwrap(),
			"52.38.27.190:9735".parse().unwrap(),
		),
		(
			"03db10aa09ff04d3568b0621750794063df401e6853c79a21a83e1a3f3b5bfb0c8".parse().unwrap(),
			"69.59.18.80:9735".parse().unwrap(),
		),
		(
			"027100442c3b79f606f80f322d98d499eefcb060599efc5d4ecb00209c2cb54190".parse().unwrap(),
			"3.226.165.222:9735".parse().unwrap(),
		),
	];

	for (node_id, address) in nodes_to_connect {
		match node.connect(node_id, address.clone(), false) {
			Ok(()) => {},
			Err(e) => {
				eprintln!("Failed to connect to node {}@{}: {}", node_id, address, e);
			},
		}
	}

	std::thread::sleep(Duration::from_secs(10));
	let count_node = Arc::clone(&node);
	std::thread::spawn(move || loop {
		let (num_om_support, num_nodes) =
			network_onion_message_support(&count_node.network_graph());
		let share_pct = num_om_support as f32 / num_nodes as f32 * 100.0;
		println!(
			"Nodes that announce OM support: {}/{} = {:.1}%",
			num_om_support, num_nodes, share_pct
		);

		if let Ok(mut file) = File::create(PERSIST_PATH) {
			let html = get_html_template(share_pct, num_om_support, num_nodes);
			match file.write_all(html.as_bytes()) {
				Ok(()) => {},
				Err(e) => {
					eprintln!("Failed to write to file: {}", e);
				},
			}
		}
		std::thread::sleep(Duration::from_secs(10));
	});

	pause();
	node.stop().unwrap();
}

fn network_onion_message_support(network_graph: &NetworkGraph) -> (usize, usize) {
	let num_nodes = network_graph
		.list_nodes()
		.iter()
		.filter(|n| network_graph.node(n).map_or(false, |n| n.announcement_info.is_some()))
		.count();
	let num_support_oms = network_graph
		.list_nodes()
		.iter()
		.filter(|n| {
			network_graph.node(n).map_or(false, |n| {
				n.announcement_info
					.as_ref()
					.map_or(false, |info| info.features().supports_onion_messages())
			})
		})
		.count();

	debug_assert!(num_support_oms <= num_nodes);
	(num_support_oms, num_nodes)
}

fn pause() {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
	write!(stdout, "Press any key to continue...").unwrap();
	stdout.flush().unwrap();

	// Read a single byte and discard
	let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_html_template(share_pct: f32, num_om_support: usize, num_nodes: usize) -> String {
	let last_update: DateTime<Utc> = SystemTime::now().into();
	format!(
		"<!DOCTYPE html>
		<html>
		<head>
		<style>
		body {{
			background-color: #D3D3D3;
			color: black;
			font-family: \"Gill Sans\", sans-serif;
			font-size: 14pt;
		}}
		.center {{
			text-align: center;
			margin: auto;
			width: 70%;
			padding: 10px;
		}}
		.center-text {{
			text-align: center;
			width: 70%;
			margin: auto;
			padding: 10px;
			background-color: white;
		}}
		.last-updated-footer {{
			position: fixed;
			left: 0;
			bottom: 0;
			width: 100%;
			text-align: center;
			font-size: 9pt;
		}}
		</style>
			</head>
			<body>
			<div class=\"center\">
			<p>Lightning nodes supporting onion message forwarding:</p>
			<p style=\"font-size: 48pt;\">{:.1}%</p>
			<p style=\"font-size: 24pt;\">({}/{} nodes)</p>
			</div>
			<div class=\"last-updated-footer\">
			Last updated: {}
		</div>
			</body>
			</html>
			</body>
			</html>",
		share_pct, num_om_support, num_nodes, last_update
	)
}
