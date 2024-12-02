const { resolve } = require('path');
const { readFileSync } = require('fs');

function parseInput(input) {
    return input
        .trim()
        .split('\n')
        .reduce(
            (acc, line) => {
                const [left, right] = line.trim().split(/\s+/);
                acc.left.push(Number(left));
                acc.right.push(Number(right));
                return acc;
            },
            { left: [], right: [] }
        );
}

function findDistance(left, right) {
    let distance = 0;

    for (let i = 0; i < left.length; i++)
        distance += Math.abs(left[i] - right[i]);

    return distance;
}

function findSimilarity(left, right) {
    let score = 0;
    for (let i = 0; i < left.length; i++) {
        let counter = 0;
        for (let j = 0; j < right.length; j++) {
            if (left[i] === right[j]) counter++;
        }
        score += left[i] * counter;
    }
    return score;
}

function main() {
    const input = readFileSync(resolve(__dirname, 'input.txt'), {
        encoding: 'utf-8',
    });
    const parsed = parseInput(input);
    const left = parsed.left.sort();
    const right = parsed.right.sort();

    const distance = findDistance(left, right);
    const similarity = findSimilarity(left, right);

    console.log(distance);
    console.log(similarity);
}
main();
