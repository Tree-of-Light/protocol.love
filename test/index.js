const tryorama = require('@holochain/tryorama')
const path = require('path')
const fs = require('fs')
// Point to your DNA file and give it a nickname.
// The DNA file can either be on your filesystem...
main()
async function main() {
	const cogov_dna_path = path.join(__dirname, `../dist/${fs.readdirSync(`${__dirname}/../dist/`)}`)
	const cogov_dna = tryorama.Config.dna(cogov_dna_path, 'cogov')
	const main_config = tryorama.Config.gen({
		cogov: cogov_dna,
	}, {
		network: {
			type: 'sim2h',
			sim2h_url: 'ws://localhost:9000',
		},
	})
	const orchestrator = new tryorama.Orchestrator()
	orchestrator.registerScenario('create_collective; get_collective', async (s, t) => {
		const { alice } = await s.players({ alice: main_config, })
		await alice.spawn({})
		// TODO: figure out consistency in test
//		const { carol } = await s.players({ carol: main_config }, true)
		const create_collective_response = await alice.call('cogov', 'cogov', 'create_collective', {
			collective: {
				name: `Collective 1`
			}
		})
		const { Ok: { collective_address, collective } } = create_collective_response
		t.assert(collective_address, 'collective_address should be truthy')
		t.assert(collective, 'collective should be truthy')
//		await s.consistency()
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
	const report = await orchestrator.run()
	console.log(report)
}
