import { NaskoInstructions } from "./instructions"
import { NaskoUtils } from "./utils"

export const yieldChunks = (arr: any[], size: number) => Array.from(
	Array(Math.ceil(arr.length / size)), (_, i) => arr.slice(i * size, i * size + size))

export class NaskoTokenizer {
	contents: string
	private _debug: boolean
	private _instructions: string[]

	constructor(contents: string, debug: boolean = false) {
		this.contents = contents
		this._debug = debug
		this._instructions = []

		this.log(`constructed with contents of length: ${contents.length}`)
	}

	private log(content: string, ...optional: any[]) {
		if(this._debug)
			NaskoUtils.log('NaskoTokenizer', content, ...optional)
	}

	parse(): number[] {
		console.log("\n")
		let line = 0
		if(!this._instructions.length) {
			this._instructions = this.contents.split(/\n+/)
		}

		let bytecode = [78, 83, 75, 79]
		let instructions = Object.getOwnPropertyNames(NaskoInstructions).filter(NaskoUtils.filterPropertyNames)

		for(const _instruction of this._instructions) {
			try {
				if(_instruction == '') continue

				let args = _instruction.split(/,?\s+/)
				let instruction = args.shift() as string
				this.log(`instruction: ${instruction}`)
				this.log(`args: [${args.map(x => `'${x}'`).join(", ")}]`)

				if(instructions.includes(instruction)) {
					// TODO: Find a better alternative
					bytecode = bytecode.concat(eval(`require("./instructions").NaskoInstructions.${instruction}(${JSON.stringify(args)})`))
				} else {
					bytecode = bytecode.concat(NaskoInstructions.hlt(args))
				}

				this.log("current bytecode: ", bytecode)

			} catch(e) {
				this.log("found error: ", e.toString())
				NaskoUtils.appendError(e, line)
			}
			console.log("\n")
			line++
		}

		let endSlice = bytecode.slice(bytecode.length - 4, bytecode.length)
		if(endSlice != [0, 0, 0, 0]) {
			this.log("missing end halt slice, pushing ending halt")
			bytecode.concat([0, 0, 0, 0])
		}

		return bytecode
	}
}
