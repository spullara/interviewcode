//
//  main.cpp
//  interviewcodecpp
//
//  Created by Sam Pullara on 1/13/17.
//  Copyright © 2017 Sam Pullara. All rights reserved.
//

#include <iostream>
#include <set>
#include <unordered_set>
#include <random>
#include <algorithm>
#include <chrono>
#include <functional>
#include <memory>

#include "entity.h"

using namespace std;

using EntitySet = unordered_set<unique_ptr<Entity>, std::hash<unique_ptr<Entity>>, Entity::UniquePtrComparator>;

thread_local vector<Entity*> _render_entities_buffer;
void render(const u32string &text, EntitySet const& entitySet, /* out */ u32string& result) {
    result.clear();
    result.reserve(text.length() * 2);

    auto entities = &_render_entities_buffer;
    entities->clear();
    entities->reserve(entitySet.size());
    for (auto const& entity: entitySet) {
        entities->push_back(entity.get());
    }
    std::sort(entities->begin(), entities->end(), Entity::PtrComparator());

    int pos = 0;
    for (auto const& entity: *entities) {
        result.append(text, pos, entity->start - pos);
        result.append(entity->html);
        pos = entity->end;
    }

    result.append(text, pos);
}

vector<EntitySet> createEntriesList(u32string text) {
    default_random_engine generator;

    uniform_int_distribution<int> distribution(0, 9);
    auto r = bind(distribution, generator);

    uniform_int_distribution<int> distribution2 = uniform_int_distribution<int>(0, (int) (text.length() - 1));
    auto r2 = bind(distribution2, generator);

    vector<EntitySet> entityList;
    entityList.reserve(1000);

    for (int i = 0; i < 1000; i++) {
        int total = r();
        EntitySet entitySet;
        entitySet.reserve(total * 2);
        auto indices = vector<int>();
        for (int j = 0; j < total * 2; j++) {
            int next;
            while(find(indices.begin(), indices.end(), next = r2()) != indices.end());
            indices.push_back(next);
        }
        sort(indices.begin(), indices.end());
        for (int j = 0; j < total * 2; j += 2) {
            int start = *next(indices.begin(), j);
            int end = *next(indices.begin(), j+1);
            int length = end - start;
            u32string html = U"";
            for (int k = 0; k < length; k++) {
                html.append(U"XX");
            }
            entitySet.insert(std::unique_ptr<Entity>(new Entity(start, end, std::move(html))));
        }
        entityList.push_back(std::move(entitySet));
    }

    return entityList;
}

long currentTimeMillis() {
    using namespace std::chrono;
    milliseconds ms = duration_cast<milliseconds>(system_clock::now().time_since_epoch());
    return ms.count();
}

void bench() {
    u32string text = U"Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    auto entityList = createEntriesList(text);

    {
        u32string result;
        for (int j = 0; j < 5; j++) {
            long start = currentTimeMillis();
            for (int i = 0; i < 1000000; i++) {
                render(text, entityList[i % 1000], result);
            }
            cout << (currentTimeMillis() - start) << " ns/op\n";
        }
    }
}

int main(int argc, const char * argv[]) {
    cout << "Starting\n";
    EntitySet entitySet;
    entitySet.insert(std::unique_ptr<Entity>(new Entity(25, 32, U"<#mobile>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(33, 42, U"<#startups>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(46, 51, U"<#OF12>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(82, 102, U"<http://t.co/HtzEMgAC>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(103, 110, U"<@TiEcon>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(111, 127, U"<@sv_entrepreneur>")));
    entitySet.insert(std::unique_ptr<Entity>(new Entity(128, 132, U"<@500>")));

    u32string text = U"Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    u32string test = U"Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";

    u32string result;
    render(text, entitySet, result);
    // cout << result << ": " << (test == result) << "\n";
    cout << "Running benchmark\n";

    bench();

    return 0;
}
