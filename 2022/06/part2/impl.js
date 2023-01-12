const fs = require('fs');

fs.readFile('input', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const windowSize = 14;

    process(data, windowSize);
});

function process(data, windowSize) {
    const len = data.length - windowSize;

    for (let i = 0; i < len; i++) {
        if (!windowContainsDupes(data, i, windowSize)) {
            console.log(i + windowSize);
            return;
        }
    }
}

function windowContainsDupes(data, offset, windowSize) {
    const a = 'a'.codePointAt(0);

    let bitmap = 0;

    for (let w = offset; w < offset + windowSize; w++) {
        const char = data.codePointAt(w) - a;
        const mask = 0x1 << char;

        if (bitmap & mask) {
           return true; 
        }
        
        bitmap |= mask;
    }

    return false;
}
