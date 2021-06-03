const { Nasko } = require('../build/nasko')
const fs = require('fs')
const config = require('./config')
const inquirer = require('inquirer')
const sleep = require('util').promisify(setTimeout)

console.clear()
inquirer
	.prompt([
		{
			type: 'list',
			name: 'test',
			message: 'What test would you like to run?',
			choices: fs.readdirSync(__dirname + '/scripts').map(sc => sc.split('.')).filter(f => ['nko', 'nasko', 'nsko'].includes(f[1])).map(f => f[0])
		}
	])
	.then(async ({ test }) => {
		try {
			await sleep(1000)
			console.clear()

			let contents = fs.readFileSync(__dirname + `/scripts/${test}.nko`).toString()
			const start = Date.now()

			let bytecode = Nasko.parse(contents, true)

			const elapsed = Date.now() - start
			fs.writeFileSync(__dirname + `/scripts/${test}.naskode`, bytecode.join(''))
			console.log(`\n${test}... \x1b[32;1mok.\x1b[0m`)
			if(config.bench)
				console.log(`Bench results: ${elapsed}ms`)
		} catch(e) {
			if(process.env.DEBUG)
				console.log(); console.log(e)
			console.log(`\n${test}... \x1b[31;1mFAILED\x1b[0m`)
		}
	})
