//
//  main.cpp
//  interviewcodecpp
//
//  Created by Sam Pullara on 1/13/17.
//  Copyright © 2017 Sam Pullara. All rights reserved.
//

#include <iostream>
#include <set>
#include <random>
#include <algorithm>
#include <chrono>
#include <functional>

#include "entity.h"

using namespace std;


u32string render(const u32string &text, set<Entity> const& entitySet) {
    u32string result = u32string();
    result.reserve(text.length() * 2);

    int pos = 0;
    for (auto const& entity: entitySet) {
        result.append(text, pos, entity.start - pos);
        result.append(entity.html);
        pos = entity.end;
    }

    result.append(text, pos);
    return result;
}

vector<set<Entity>> createEntriesList(u32string text) {
    default_random_engine generator;

    uniform_int_distribution<int> distribution(0, 9);
    auto r = bind(distribution, generator);

    uniform_int_distribution<int> distribution2 = uniform_int_distribution<int>(0, (int) (text.length() - 1));
    auto r2 = bind(distribution2, generator);

    vector<set<Entity>> entityList;

    for (int i = 0; i < 1000; i++) {
        set<Entity> entitySet;
        int total = r();
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
            Entity entity(start, end, std::move(html));
            entitySet.insert(std::move(entity));
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
        for (int j = 0; j < 5; j++) {
            long start = currentTimeMillis();
            for (int i = 0; i < 1000000; i++) {
                render(text, entityList[i % 1000]);
            }
            cout << (currentTimeMillis() - start) << " ns/op\n";
        }
    }
}

int main(int argc, const char * argv[]) {
    cout << "Starting\n";
    set<Entity> entitySet;
    entitySet.insert(Entity(25, 32, U"<#mobile>"));
    entitySet.insert(Entity(33, 42, U"<#startups>"));
    entitySet.insert(Entity(46, 51, U"<#OF12>"));
    entitySet.insert(Entity(82, 102, U"<http://t.co/HtzEMgAC>"));
    entitySet.insert(Entity(103, 110, U"<@TiEcon>"));
    entitySet.insert(Entity(111, 127, U"<@sv_entrepreneur>"));
    entitySet.insert(Entity(128, 132, U"<@500>"));
    
    u32string text = U"Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    u32string test = U"Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";

    u32string result = render(text, entitySet);
    // cout << result << ": " << (test == result) << "\n";
    cout << "Running benchmark\n";
    
    bench();
    
    return 0;
}
