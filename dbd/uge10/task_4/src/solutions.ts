import data from "./comics.json";


const comics = data.comics;

function subtask01(numbers: number[]): number[] {
    // 1. Map the list of numbers to a list of their square roots: [1, 9, 16, 100]
    return numbers.map(x => Math.sqrt(x));
}

function subtask02(words: string[]): string[] {
    // 2. Map the list of words so each is wrapped in a h1 tag: ["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]
    return words.map(x => "<h1>" + x + "</h1>");
}

function subtask03(words: string[]): string[] {
    // 3. Use map to uppercase the words (all letters): ["I'm", "yelling", "today"]
    return words.map(x => x.toUpperCase());
}

function subtask04(words: string[]): number[] {
    // 4. Use map to transform words into their lengths: ["I", "have", "loooooong", "words"]
    return words.map(x => x.length);
}

function subtask05(): string[] {
    // 5. Get the json file comics.json from the course site. Paste it into your browser's Javascript console. Use map to get the image urls, and wrap them in img-tags.
    return comics.map(x => "<img>" + x.img + "</img>");
}

function subtask06(numbers: number[]): number {
    // 6. Use reduce to sum the array of numbers: [1,2,3,4,5]
    const reducer = (accumulator: number, curr: number) => accumulator + curr;

    return numbers.reduce(reducer);
}

function subtask07(dict: any): number {
    return dict.reduce((acc: any, curr: any) => acc + curr.x, 0);
}

function subtask08() {
    // 8. Use reduce to flatten an array of arrays: [[1,2] [3,4], [5,6]]
    let arr = [[1,2], [3,4], [5,6]]
    const reducer = (modifier: any, curr: any) => modifier.concat(curr);
    return arr.reduce(reducer);
}

function subtask09() {
    // 9. Use reduce to return an array of the positive numbers: [-3, -1, 2, 4, 5]
    let arr = [-3, -1, 2, 4, 5];
    const reducer = (modifier: any, curr: any) => modifier.concat(Math.abs(curr));

    return arr.reduce(reducer, []);
}

console.log("1:");
console.log(subtask01([1, 9, 16, 100]));

console.log("2: ");
console.log(subtask02(["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]));

console.log("3: ");
console.log(subtask03(["I'm", "yelling", "today"]));

console.log("4: ");
console.log(subtask04(["I", "have", "loooooong", "words"]));

console.log("5: ");
console.log(subtask05());

console.log("6: ");
console.log(subtask06([1, 2, 3, 4, 5]));

console.log("7: ");
console.log(subtask07([{ x: 1 }, { x: 2 }, { x: 3 }]));

console.log("8: ");
console.log(subtask08());

console.log("9: ");
console.log(subtask09());

