import { NaskoTokenizer } from './tokenizer'
import { NaskoUtils } from './utils'

export class Nasko {
	static parse(contents: string, debug: boolean = false): string[] {
		let t = new NaskoTokenizer(contents, debug)
		let bytecode = t.parse()
		return bytecode.map(byte => NaskoUtils.padZero(byte, 3))
	}
}
