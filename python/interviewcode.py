import random
import time

text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon " \
       "@sv_entrepreneur @500!"

def create_entity_list():
    length = len(text)
    entityList = []
    for i in range(0, 1000):
        entities = []
        total = random.randint(0, 9)
        indices = []
        for j in range(0, total*2):
            while True:
                next = random.randint(0, length - 1)
                if next not in indices:
                    indices.append(next)
                    break
        indices.sort()
        for j in range(0, total):
            start = indices[j*2]
            end = indices[j*2 + 1]
            l = end - start
            sb = ""
            for _ in range(0, l):
                sb += "XX"
            entities.append((start, end, sb))
        entityList.append(entities)
    return entityList

def testEntites():
    entities = []
    entities.append((82, 102, "<http://t.co/HtzEMgAC>"))
    entities.append((103, 110, "<@TiEcon>"))
    entities.append((25, 32, "<#mobile>"))
    entities.append((33, 42, "<#startups>"))
    entities.append((46, 51, "<#OF12>"))
    entities.append((111, 127, "<@sv_entrepreneur>"))
    entities.append((128, 132, "<@500>"))
    return entities

def render(text, entities):
    entityList = entities.copy()
    entityList.sort(key = lambda x : x[0])
    pos = 0
    sb = ""
    for entity in entityList:
        sb += text[pos : entity[0]]
        sb += entity[2]
        pos = entity[1]
    sb += text[pos :]
    return sb

if __name__ == "__main__":
    result = render(text, testEntites())
    expected = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!"
    print(result)
    print(result == expected)
    entityList = create_entity_list()

    total = 1000000
    for _ in range(0, 5):
        start = time.clock_gettime_ns(time.CLOCK_MONOTONIC)
        for i in range(0, total):
            render(text, entityList[i % 1000])
        end = time.clock_gettime_ns(time.CLOCK_MONOTONIC)
        diff = end - start
        print(str(int(diff/total)) + " ns/render")
