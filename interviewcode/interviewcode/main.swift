//
//  main.swift
//  interviewcode
//
//  Created by Sam Pullara on 1/10/17.
//  Copyright © 2017 Sam Pullara. All rights reserved.
//

import Foundation

var text: String = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
"http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";

struct Entity : Hashable, Comparable {
    var start: Int
    var end: Int
    var html: String
    
    var hashValue: Int {
        return html.hashValue ^ start ^ end
    }
    
    static func == (lhs: Entity, rhs: Entity) -> Bool {
        return lhs.start == rhs.start && lhs.end == rhs.end && lhs.html == rhs.html
    }
    
    static func <(lhs: Entity, rhs: Entity) -> Bool {
        return lhs.start < rhs.start
    }
}


func createEntityList(text: String) -> Array<Set<Entity>> {
    let length = text.count
    var entitiesList: Array<Set<Entity>> = Array()
    for _ in 1...1000 {
        var entities: Set<Entity> = Set()
        let total = Int(arc4random() % 10)
        var indices: Array<Int> = Array()
        for _ in 0..<total*2 {
            var next: Int
            repeat {
                next = Int(arc4random()) % length
            } while (indices.contains(next))
            indices.append(next)
        }
        indices.sort()
        for j in 0..<total {
            let start = indices[j*2]
            let end = indices[j*2 + 1]
            let len = end - start
            var sb = String()
            for _ in 1...len {
                sb.append("XX")
            }
            entities.insert(Entity(start: start, end: end, html: sb))
        }
        entitiesList.append(entities)
    }
    return entitiesList
}

func testEntities() -> Set<Entity> {
    var entities: Set<Entity> = Set()
    entities.insert(Entity(start: 25, end: 32, html: "<#mobile>"))
    entities.insert(Entity(start: 33, end: 42, html: "<#startups>"))
    entities.insert(Entity(start: 46, end: 51, html: "<#OF12>"))
    entities.insert(Entity(start: 82, end: 102, html: "<http://t.co/HtzEMgAC>"))
    entities.insert(Entity(start: 103, end: 110, html: "<@TiEcon>"))
    entities.insert(Entity(start: 111, end: 127, html: "<@sv_entrepreneur>"))
    entities.insert(Entity(start: 128, end: 132, html: "<@500>"))
    return entities
}

func render(text: String, entities: Set<Entity>) -> String {
    let entityArray = Array(entities).sorted()
    var sb = String()
    var pos = 0
    var posIndex = text.startIndex
    for entity in entityArray {
        let startIndex = text.index(posIndex, offsetBy: entity.start - pos)
        sb.append(contentsOf: text[posIndex ..< startIndex])
        sb += entity.html
        posIndex = text.index(startIndex, offsetBy: entity.end - entity.start)
        pos = entity.end
    }
    sb.append(contentsOf: text[posIndex...])
    return sb
}

func time() -> UInt64 {
    var darwinTime : timeval = timeval(tv_sec: 0, tv_usec: 0)
    gettimeofday(&darwinTime, nil)
    return UInt64(darwinTime.tv_sec * 1000) + UInt64(darwinTime.tv_usec / 1000)
}

var entityList = createEntityList(text: text)
print(render(text: text, entities: testEntities()))

var entitiesList = createEntityList(text: text)

for _ in 1...2 {
    let start = time()
    for i in 1...1000000 {
        render(text: text, entities: entitiesList[i % 1000])
    }
    print(time() - start)
}
