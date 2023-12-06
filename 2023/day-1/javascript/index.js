/**
 * Dependencies
 */

// NodeJS Library
const fs = require('fs')
const { resolve } = require('path')

// Constants

const inputPath = resolve(__dirname, '../input.txt')
const pattern = /(one|two|three|four|five|six|seven|eight|nine|\d)/g
const stringNumbers = {
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
}

/**
 * Helpers
 */

const findNumbers = (str) => str.match(pattern).map((e) => (stringNumbers[e] ? stringNumbers[e] : Number(e)))

/**
 * Main
 */

function main() {
    const input = fs.readFileSync(inputPath, { encoding: 'utf-8' })
    const numbers = input
        .trim()
        .split('\n')
        .map((line) => findNumbers(line.trim()))
    const values = numbers.map((e) => Number(`${e.at(0)}${e.at(-1)}`))
    console.log(`Solution: ${values.reduce((sum, e) => (sum += e), 0)}`)
}
main()
