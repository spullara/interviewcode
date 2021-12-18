const text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!"

function getEntities() {
    return [
        {start: 25, end: 32, html: "<#mobile>"},
        {start: 33, end: 42, html: "<#startups>"},
        {start: 46, end: 51, html: "<#OF12>"},
        {start: 82, end: 102, html: "<http://t.co/HtzEMgAC>"},
        {start: 103, end: 110, html: "<@TiEcon>"},
        {start: 111, end: 127, html: "<@sv_entrepreneur>"},
        {start: 128, end: 132, html: "<@500>"},
    ];
}

function createEntriesList() {
    const entitiesList = [];
    for (let i = 0; i < 1000; i++) {
        const entities = [];
        const total = Math.random() * 10;
        const indices = [];
        for (var j = 0; j < total * 2; j++) {
            let next;
            while (indices.includes(next = Math.floor(Math.random() * text.length))) ;
            indices.push(next);
        }
        indices.sort();
        for (j = 0; j < total * 2; j += 2) {
            const start = indices[j];
            const end = indices[j + 1];
            const length = end - start;
            let sb = "";
            for (let k = 0; k < length; k++) {
                sb += "XX";
            }
            entities.push({start, end, html: sb});
        }
        entitiesList.push(entities);
    }
    return entitiesList;
}

function render(text, unsortedEntities) {
    const entities = unsortedEntities.sort((o1, o2) => o1.start - o2.start);
    let result = '';
    const arr = Array.from(text)
    const arrLen = arr.length
    let pos = 0
    for (let entity of entities) {
        const start = entity.start;
        for (; pos < arrLen; pos++) {
            // If this is the start of an entity add it to the result
            if (start === pos) {
                result += entity.html;
                pos = entity.end;
                break;
            } else {
                result += arr[pos];
            }
        }
    }
    for (; pos < arrLen; pos++) {
        result += arr[pos]
    }
    return result;
}

function renderInvert(text, unsortedEntities) {
    const entities = [...unsortedEntities.sort((o1, o2) => o1.start - o2.start)];
    let result = '';
    let entity = entities.shift();
    if (!entity) return text;
    let start = entity.start;
    const codePoints = Array.from(text);
    const length = codePoints.length;
    for (let pos = 0; pos < length; pos++) {
        if (start === pos) {
            result += entity.html;
            pos = entity.end - 1;
            entity = entities.shift();
            if (entity) {
                start = entity.start;
            } else {
                start = -1;
            }
        } else {
            result += codePoints[pos];
        }
    }
    return result;
}


function renderMark(text, unsortedEntities) {
    const entities = [...unsortedEntities.sort((o1, o2) => o1.start - o2.start)];

    let result = '';
    const arr = Array.from(text)
    const arrLen = arr.length
    // let entityNum = 0

    let i = 0

    if (entities.length) {
        for (; i < arrLen; i++) {
            let didAddEntity = false
            // If this is the start of an entity add it to the result
            while (entities[0].start === i) {
                const entity = entities.shift();
                result += entity.html;
                i = entity.end - 1
                didAddEntity = true
                if (!entities.length) {
                    break
                }
            }

            if (didAddEntity) {
                if (entities.length === 0) {
                    i++;
                    break
                }
            } else {
                result += arr[i]
            }
        }
    }

    for (; i < arrLen; i++) {
        result += arr[i]
    }

    return result
}

function offsetByCodePoints(s, index, codePointOffset) {
    const length = s.length;
    let x = index;
    for (let i = 0; i < length && i < codePointOffset; i++) {
        const ch = s.charAt(x++);
        if (ch >= '\ud800' && ch < '\udc00') {
            const ch1 = s.charAt(x);
            if (ch1 >= '\udc00' && ch1 < '\ue000') {
                x++;
            }
        }
    }
    return x;
}

function renderJava(text, unsortedEntities) {
    const entities = unsortedEntities.sort((o1, o2) => o1.start - o2.start);
    let result = '';
    let pos = 0;
    let codePointPosition = 0;
    for (let entity of entities) {
        const start = offsetByCodePoints(text, pos, entity.start - codePointPosition);
        for (let i = pos; i < start; i++) {
            result += text[i];
        }
        result += entity.html;
        codePointPosition = entity.end;
        pos = offsetByCodePoints(text, start, entity.end - entity.start);
    }
    for (let i = pos; i < text.length; i++) {
        result += text[i];
    }
    return result;
}

function bench(name, func) {
    const text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
        "http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    const entitiesList = createEntriesList();

    for (var j = 0; j < 5; j++) {
        for (var i = 0; i < 10000; i++) {
            func(text, entitiesList[i % 1000]);
        }
        const start = new Date().getTime();
        for (i = 0; i < 1000000; i++) {
            func(text, entitiesList[i % 1000]);
        }
        let elapsed = new Date().getTime() - start;
        console.log(name + ": " + elapsed + "ns per render");
    }
}

console.log(renderInvert(text, getEntities()));

const expected = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  " +
    "<http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
console.log(renderMark(text, getEntities()) === expected);
console.log(render(text, getEntities()) === expected);
console.log(renderJava(text, getEntities()) === expected);
console.log(renderInvert(text, getEntities()) === expected);

bench("render by invert", renderInvert);
bench("render by mark", renderMark);
bench("render by character", render);
bench("render by java", renderJava);
