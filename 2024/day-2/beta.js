class Beta {
    input

    constructor(input) {
        this.input = input
    }

    _parse() {
        const reports = this.input
            .trim()
            .split('\n')
            .map((line) =>
                line
                    .trim()
                    .split(' ')
                    .map((level) => Number(level))
            )
        return reports
    }

    _isIncreasing(report) {
        return report.reduce((acc, value, index, array) => {
            if (!acc) return false
            if (index == 0) return true
            return value > array[index - 1]
        }, true)
    }

    _isDecreasing(report) {
        return report.reduce((acc, value, index, array) => {
            if (!acc) return false
            if (index == 0) return true
            return value < array[index - 1]
        }, true)
    }

    _isDifferent(report) {
        return report.reduce((acc, value, index, array) => {
            if (!acc) return false
            if (index == 0) return true
            const difference = Math.abs(value - array[index - 1])
            return difference >= 1 && difference <= 3
        }, true)
    }

    _solve() {
        const reports = this._parse()
        const safe = reports.filter((report) => {
            if (
                (this._isIncreasing(report) || this._isDecreasing(report)) &&
                this._isDifferent(report)
            )
                return true

            for (let i = 0; i < report.length; i++) {
                const subset = report.slice(0, i).concat(report.slice(i + 1))
                if (
                    (this._isIncreasing(subset) ||
                        this._isDecreasing(subset)) &&
                    this._isDifferent(subset)
                )
                    return true
            }

            return false
        })

        return safe.length
    }

    solve() {
        const answer = this._solve()

        console.log(`Beta: ${answer}`)
    }
}

module.exports = { Beta }
