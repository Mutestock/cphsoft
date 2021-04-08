//import MyModule from "resources/comics.json';


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

const comics = [
    {
        "month": "1",
        "num": 43,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Red Spiders 2",
        "transcript": "[[Red spiders, with round appendages at the end of each of their six legs, are seen navigating an environment of blocks and other geometric constructions]]\n{{title text: This was actually drawn years before Red Spiders}}",
        "alt": "This was actually drawn years before Red Spiders",
        "img": "https://imgs.xkcd.com/comics/red_spiders_2.jpg",
        "title": "Red Spiders 2",
        "day": "1"
    },
    {
        "month": "1",
        "num": 44,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Love",
        "transcript": "[[A man and a woman stand facing one another]]\nMan: I love you!\nWoman: I love you!\nMan: I love you more!\nWoman: Yeah.\n[[A man and a woman stand facing one another - saying nothing.]]\n{{Alt-text: This one makes me wince every time I think about it}}",
        "alt": "This one makes me wince every time I think about it",
        "img": "https://imgs.xkcd.com/comics/love.jpg",
        "title": "Love",
        "day": "1"
    },
    {
        "month": "1",
        "num": 45,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Schrodinger",
        "transcript": "[[Label: Schrödinger's Comic]]\n[[Two figures standing, one with a black hat]]\nThe last panel of this comic is both funny and not funny at the same time.\nUntil you read it, there's no way to tell which it will end up being.\nShit.\n{{alt: There was no alt-text until you moused over}}",
        "alt": "There was no alt-text until you moused over",
        "img": "https://imgs.xkcd.com/comics/schrodinger.jpg",
        "title": "Schrodinger",
        "day": "4"
    },
    {
        "month": "1",
        "num": 46,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Secrets",
        "transcript": "I just want you to share in my secrets\n[[lonely looking girl staring down]]\nand not run away\n{{alt: I'm a big fan of Kurt Halsey}}",
        "alt": "I'm a big fan of Kurt Halsey",
        "img": "https://imgs.xkcd.com/comics/secrets.jpg",
        "title": "Secrets",
        "day": "6"
    },
    {
        "month": "1",
        "num": 47,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Counter-Red Spiders",
        "transcript": "[[A stack of stick figures, standing on each others shoulders extends from the bottom of the frame to the top.  Cuboids hang in the air]]\nThe counter-red-spider offensive begins ...\n{{title text: I hope we can stop them}}",
        "alt": "I hope we can stop them",
        "img": "https://imgs.xkcd.com/comics/counter-red-spiders.jpg",
        "title": "Counter-Red Spiders",
        "day": "9"
    },
    {
        "month": "1",
        "num": 48,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Found",
        "transcript": "[[A male and female stick figure are standing on a white hill (presumably snow) with a grey sky covered with thick streaks of white, and small pink dots]]\nwe are just two people \nwho found each other\n{{No more, no less}}",
        "alt": "No more, no less",
        "img": "https://imgs.xkcd.com/comics/found.jpg",
        "title": "Found",
        "day": "12"
    },
    {
        "month": "1",
        "num": 49,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Want",
        "transcript": "I want to be brave enough to tell you how I feel.\nI want to say \"I love you\" _before_ I hang up the phone for once.\nI want to drive all night with you, listening to mix tapes, not caring where we end up.\nOh, and I also really want to get with your sister.\nI mean, DAMN.\n{{title text: Well, she's pretty hot.}}",
        "alt": "Well, she's pretty hot.",
        "img": "https://imgs.xkcd.com/comics/want.jpg",
        "title": "Want",
        "day": "14"
    },
    {
        "month": "1",
        "num": 50,
        "link": "",
        "year": "2006",
        "news": "",
        "safe_title": "Penny Arcade",
        "transcript": "Tycho: You know what? If you've never played the 1995 SNES RPG 'Seiken Densetsu' don't even _bother_ reading today's strip. We don't _need_ your kind here.\n{{title text: Of course, Penny Arcade has already mocked themselves for this. They don't care.}}",
        "alt": "Of course, Penny Arcade has already mocked themselves for this.  They don't care.",
        "img": "https://imgs.xkcd.com/comics/penny_arcade.jpg",
        "title": "Penny Arcade",
        "day": "17"
    }
]

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
}

console.log(subtask01([1, 9, 16, 100]));
console.log(subtask02(["Intro", "Requirements", "Analysis", "Implementation", "Conclusion", "Discussion", "References"]));
console.log(subtask03(["I'm", "yelling", "today"]));
console.log(subtask04(["I", "have", "loooooong", "words"]));
console.log(subtask05());
console.log(subtask06([1, 2, 3, 4, 5]));
console.log(subtask07([{ x: 1 }, { x: 2 }, { x: 3 }]));
console.log(subtask08());


