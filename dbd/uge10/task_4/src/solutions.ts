

const greetings: string = "hello";
const numbers: number[] = [1, 2, 3];

function subtask01(numbers: number[]): number[] {
    // 1. Map the list of numbers to a list of their square roots: [1, 9, 16, 100]
    return numbers.map(x => Math.sqrt(x));
}

function subtask02(words: string[]): string[] {
    // 2. Map the list of words so each is wrapped in a h1 tag: ["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]
    return words.map(x => "<h1>"+x+"</h1>");
}

function subtask03() {
    // 3. Use map to uppercase the words (all letters): ["I'm", "yelling", "today"]
}

function subtask04() {
    // 4. Use map to transform words into their lengths: ["I", "have", "loooooong", "words"]
}

function subtask05() {
    // 5. Get the json file comics.json from the course site. Paste it into your browser's Javascript console. Use map to get the image urls, and wrap them in img-tags.
}

function subtask06() {
    // 6. Use reduce to sum the array of numbers: [1,2,3,4,5]
}

function subtask07() {
    // 7. Use reduce to sum the x-value of the objects in the array: [{x: 1}, {x: 2}, {x: 3}]
}

function subtask08() {
    // 8. Use reduce to flatten an array of arrays: [[1,2] [3,4], [5,6]]
}

function subtask09() {
    // 9. Use reduce to return an array of the positive numbers: [-3, -1, 2, 4, 5]
}

console.log(subtask01([1, 9, 16, 100]));
console.log(subtask02( ["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]));
