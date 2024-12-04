class Alpha {
    input

    constructor(input) {
        this.input = input
    }

    _parse() {
        const pattern = /mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)/g
        const operations = []
        for (const match of this.input.matchAll(pattern))
            operations.push({ ...match.groups })

        return operations
    }

    _compute(operation) {
        return operation.x * operation.y
    }

    _solve() {
        const operations = this._parse()
        const sum = operations.reduce(
            (acc, operation) => (acc += this._compute(operation)),
            0
        )

        return sum
    }

    solve() {
        const answer = this._solve()

        console.log(`Alpha: ${answer}`)
    }
}

module.exports = { Alpha }
