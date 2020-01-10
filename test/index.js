const tryorama = require('@holochain/tryorama')
const path = require('path')
const fs = require('fs')
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main()
async function main() {
	const cogov_dna_path = path.join(__dirname, `../dist/${fs.readdirSync(`${__dirname}/../dist/`)}`)
	const cogov_dna = tryorama.Config.dna(cogov_dna_path, 'cogov')
	// ... or on the web
	//const dnaChat = Config.dna('https://url.to/your/chat.dna.json', 'chat')
	// Set up a Conductor configuration using the handy `Conductor.config` helper.
	// Read the docs for more on configuration.
	const main_config = tryorama.Config.gen({
		cogov: cogov_dna,
	}, {
		// specify a bridge from chat to blog
		//		bridges: [Config.bridge('bridge-name', 'chat', 'blog')],
		// use a sim2h network (see conductor config options for all valid network types)
		network: {
			type: 'sim2h',
			sim2h_url: 'ws://localhost:9000',
		},
	})
	// Instatiate a test orchestrator.
	// It comes loaded with a lot default behavior which can be overridden, including:
	// * custom conductor spawning
	// * custom test result reporting
	// * scenario middleware, including integration with other test harnesses
	const orchestrator = new tryorama.Orchestrator()
	// Register a scenario, which is a function that gets a special API injected in
	orchestrator.registerScenario('create_collective; get_collective', async (s, t) => {
		// Declare two players using the previously specified config,
		// and nickname them "alice" and "bob"
		const { alice } = await s.players({ alice: main_config, })
		// You have to spawn the conductors yourself...
		await alice.spawn({})
		// ...unless you pass `true` as an extra parameter,
		// in which case each conductor will auto-spawn
		// TODO: figure out consistency in test
//		const { carol } = await s.players({ carol: main_config }, true)
		// // You can also kill them...
		// await alice.kill()
		// // ...and re-spawn the same conductor you just killed
		// await alice.spawn({})
		// // now you can make zome calls,
		const create_collective_response = await alice.call('cogov', 'cogov', 'create_collective', {
			collective: {
				name: `Collective 1`
			}
		})
		const { Ok: { collective_address, collective } } = create_collective_response
		t.assert(collective_address, 'collective_address should be truthy')
		t.assert(collective, 'collective should be truthy')
		// you can wait for total consistency of network activity,
//		await s.consistency()
		// and you can make assertions using tape by default
//		const messages = await carol.call('cogov', 'cogov', 'get_collective', {
		const messages = await alice.call('cogov', 'cogov', 'get_collective', {
			collective_address,
		})
		t.deepEqual(messages, {
				Ok: {
					collective_address,
					collective,
				}
			}
		)
	})
	// Run all registered scenarios as a final step, and gather the report,
	// if you set up a reporter
	const report = await orchestrator.run()
	// Note: by default, there will be no report
	console.log(report)
}
