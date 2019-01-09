package main

import (
	"math/rand"
	"sort"
	"strconv"
	"strings"
	"time"
)

var text string = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  " +
"http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!"

func contains(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}

type Entity struct {
	start int
	end int
	html string
}

func testEntities() map[Entity]bool {
	entities := make(map[Entity]bool)
	entities[Entity{25, 32, "<#mobile>"}] = true
	entities[Entity{33, 42, "<#startups>"}] = true
	entities[Entity{46, 51, "<#OF12>"}] = true
	entities[Entity{82, 102, "<http://t.co/HtzEMgAC>"}] = true
	entities[Entity{103, 110, "<@TiEcon>"}] = true
	entities[Entity{111, 127, "<@sv_entrepreneur>"}] = true
	entities[Entity{128, 132, "<@500>"}] = true
	return entities
}


func createEntityList() []map[Entity]bool {
	length := len(text)
	entitiesList := make([]map[Entity]bool, 1000)
	for i := 0; i < 1000; i++ {
		entities := make(map[Entity]bool)
		total := rand.Intn(10)
		indices := make([]int, total * 2)
		for j := 0; j < total * 2; j++ {
			var next int
			for {
				next = rand.Intn(length)
				if !contains(indices, next) {
					indices[j] = next
					break
				}
			}
		}
		sort.Ints(indices)
		for j := 0; j < total; j++ {
			start := indices[j*2]
			end := indices[j*2 + 1]
			length := end - start;
			sb := ""
			for k := 0; k < length; k++ {
				sb += "XX"
			}
			entities[Entity{start, end, sb}] = true
		}
		entitiesList[i] = entities
	}
	return entitiesList
}

func render(text string, entities map[Entity]bool) string {
	keys := make([]Entity, 0, len(entities))
	for k := range entities {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(o1 int, o2 int) bool {
		if keys[o1].start < keys[o2].start {
			return true
		}
		return false
	})
	sb := strings.Builder{}
	sb.Grow(len(text) * 2)
	pos := 0
	for i := range keys {
		sb.WriteString(text[pos : keys[i].start])
		sb.WriteString(keys[i].html)
		pos = keys[i].end
	}
	sb.WriteString(text[pos:])
	return sb.String()
}

func main() {
	result := render(text, testEntities())
	println(result)
	println(result == "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  " +
		"<http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!")

	entitiesList := createEntityList()

	for i := 0; i < 5; i++ {
		start := time.Now().UnixNano()
		total := 1000000
		for j := 0; j < total; j++ {
			render(text, entitiesList[j % 1000])
		}
		end := time.Now().UnixNano()
		diff := (end - start)
		nanos := int64(diff / int64(total))
		println(strconv.FormatInt(nanos, 10) + " ns/render")
	}
}
