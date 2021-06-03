import { NaskoUtils } from './utils'

export class NaskoInstructions {
	static hlt(_: string[]): number[] {
		return [0, 0, 0, 0]
	}

	static load(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(1)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode = bytecode.concat(NaskoUtils.hexify(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static add(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(2)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))
		bytecode.push(NaskoUtils.processRegister(args[2]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static sub(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(3)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))
		bytecode.push(NaskoUtils.processRegister(args[2]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static mul(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(4)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))
		bytecode.push(NaskoUtils.processRegister(args[2]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static div(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(5)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))
		bytecode.push(NaskoUtils.processRegister(args[2]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static eq(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(6)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static neq(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(7)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static jmp(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(8)
		bytecode.push(NaskoUtils.processRegister(args[0]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static jmpf(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(9)
		bytecode.push(NaskoUtils.processRegister(args[0]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static jmpb(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(10)
		bytecode.push(NaskoUtils.processRegister(args[0]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static gt(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(11)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static gte(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(12)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static lt(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(13)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

	static lte(args: string[]): number[] {
		let bytecode: number[] = []

		bytecode.push(14)
		bytecode.push(NaskoUtils.processRegister(args[0]))
		bytecode.push(NaskoUtils.processRegister(args[1]))

		return NaskoUtils.padZeroEnd(bytecode)
	}

}
