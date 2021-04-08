"use strict";
var greetings = "hello";
var numbers = [1, 2, 3];
function subtask01(numbers) {
    // 1. Map the list of numbers to a list of their square roots: [1, 9, 16, 100]
    return numbers.map(function (x) { return Math.sqrt(x); });
}
function subtask02(words) {
    // 2. Map the list of words so each is wrapped in a h1 tag: ["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]
    return words.map(function (x) { return "<h1>" + x + "</h1>"; });
}
function subtask03(words) {
    // 3. Use map to uppercase the words (all letters): ["I'm", "yelling", "today"]
    return words.map(function (x) { return x.toUpperCase(); });
}
function subtask04(words) {
    // 4. Use map to transform words into their lengths: ["I", "have", "loooooong", "words"]
    return words.map(function (x) { return x.length; });
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
console.log(subtask02(["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]));
console.log(subtask03(["I'm", "yelling", "today"]));
console.log(subtask04(["I", "have", "loooooong", "words"]));
//console.log(subtask05);
