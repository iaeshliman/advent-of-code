const { resolve } = require('path')
const { readFileSync } = require('fs')
const { Alpha } = require('./alpha')
const { Beta } = require('./beta')

const example = false

function main() {
    console.log('Advent of Code - Day 2')
    const input = readFileSync(
        resolve(__dirname, example ? 'example.txt' : 'input.txt'),
        {
            encoding: 'utf-8',
        }
    )

    const alpha = new Alpha(input)
    alpha.solve()

    const beta = new Beta(input)
    beta.solve()
}
main()
