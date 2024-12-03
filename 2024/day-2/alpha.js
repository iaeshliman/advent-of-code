class Alpha {
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

    _isConsistent(report) {
        const direction =
            report[1] - report[0] > 0 ? 'INCREASING' : 'DECREASING'
        let last = report[0]
        for (let i = 2; i < report.length; i++) {
            if (direction === 'INCREASING' && report[i] <= last) return false
            if (direction === 'DECREASING' && report[i] >= last) return false
            last = report[i]
        }
        return true
    }

    _isDifferent(report) {
        let last = report[0]
        for (let i = 1; i < report.length; i++) {
            const difference = Math.abs(report[i] - last)
            if (difference < 1 || difference > 3) return false
            last = report[i]
        }

        return true
    }

    _solve() {
        const reports = this._parse()
        const safeReports = reports.filter(
            (report) => this._isConsistent(report) && this._isDifferent(report)
        )

        return safeReports.length
    }

    solve() {
        const answer = this._solve()

        console.log(`Alpha: ${answer}`)
    }
}

module.exports = { Alpha }
