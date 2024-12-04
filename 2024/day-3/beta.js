class Beta {
    input

    constructor(input) {
        this.input = input
    }

    _parse() {
        const pattern =
            /(?:mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?:(?<modifier>do|don't)\(\))/g
        const operations = []
        let enabled = true
        for (const match of this.input.matchAll(pattern)) {
            if (match.groups.modifier != null)
                enabled = match.groups.modifier == 'do'
            if (match.groups.x != null && match.groups.y != null && enabled) {
                operations.push({ x: match.groups.x, y: match.groups.y })
            }
        }

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

        console.log(`Beta: ${answer}`)
    }
}

module.exports = { Beta }
