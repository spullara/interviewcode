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
    const codepoints = Array.from(text)
    const length = codepoints.length;
    let hasEntities = entities.length;
    OUTER:
    for (let i = 0; i < length; i++) {
        // If this is the start of an entity add it to the result
        while (hasEntities && entities[0].start === i) {
            const entity = entities.shift();
            hasEntities = entities.length;
            result += entity.html;
            i = entity.end - 1;
            continue OUTER;
        }
        result += codepoints[i]
    }

    return result;
}

function bench(name, func) {
    const text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
        "http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    const entitiesList = createEntriesList();

    for (var j = 0; j < 10; j++) {
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

const rendered = render(text, getEntities());
console.log(rendered);
const expected = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  " +
    "<http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";
console.log(rendered === expected);

bench("render by character", render);
