import chalk from "chalk"
import { range as lodashRange } from "lodash"

const errs: Array<[Error, number]> = []

export class NaskoUtils {
	static get instance() {
		return 'NaskoUtils'
	}

	static log(instance: string, content: string, ...optional: any[]) {
		const time = new Date(Date.now())
		const formatted = NaskoUtils.parseTime(time)
		console.log(`${formatted} [${instance}] ${content}`, ...optional)
	}

	static padZero(n: number | string, len: number = 2): string {
		let ns = String(n)
		return len - ns.length == 0 ? ns : `${lodashRange(0, len - ns.length).map(_ => "0").join('')}${n}`
	}

	static padZeroEnd(arr: number[], len: number = 4): number[] {
		return len - arr.length == 0 ? arr : arr.concat(lodashRange(0, len - arr.length).map(_ => 0))
	}

	static parseTime(time: Date): string {
		return `${this.padZero(time.getHours())}:${this.padZero(time.getMinutes())}:${this.padZero(time.getSeconds())}.${this.padZero(time.getMilliseconds(), 3)}`
	}

	static hexify(arg: string) {
		let n = Number(arg)
		if(n > 65535)
			throw new NumberTooLarge(`number too large: ${n}`)

		let hex = NaskoUtils.padZero(n.toString(16), 4)
		NaskoUtils.log(NaskoUtils.instance, `parsing hex bytes: ${hex}`)

		return [hex.slice(0, 2), hex.slice(2, 4)].map(n => parseInt(n, 16))
	}

	static appendError(err: Error, line: number) {
		errs.push([err, line])
	}

	static processErrors() {
		if(errs.length) {
			for(const [err, line] of errs) {
				console.log(NaskoUtils.parseError(err, line))
			}
			process.exit(1)
		}
	}

	static errString(errString: string, line: number) {
		`${chalk.redBright('error:')} ${chalk.whiteBright(errString)} ${line == 0 ? '' : chalk.cyanBright(`Line ${line}`)}`
	}

	static parseError(e: NaskoError, line: number) {
		const err = e.constructor.name

		switch(err) {
			case 'NaskoError':
				console.log(NaskoUtils.errString('', line))
		}
	}

	static processRegister(arg: string): number {
		let register = arg.replace(/\D+/g, '')
		if(!NaskoUtils.verifyRegister(register))
			throw new InvalidRegister(`Invalid register: '${register}'`)

		return Number(register)
	}

	static verifyRegister(register_: string | number): boolean {
		let register = Number(register_)
		return ((register <= 32 && register >= 63) || (register <= 31))
	}

	static filterPropertyNames(n: string) {
		const invalidPropertyNames = [
			'length', 'name', 'prototype'
		]

		return !invalidPropertyNames.includes(n)
	}
}

export class NaskoError extends Error {}
export class InvalidRegister extends NaskoError {}
export class NumberTooLarge extends NaskoError {}
