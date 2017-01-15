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


string render(string text, set<Entity> *entitySet) {
    string result = string();
    result.reserve(text.length() * 2);
    auto entityList = vector<Entity>(entitySet->begin(), entitySet->end());
    sort(entityList.begin(), entityList.end());
    int pos = 0;
    for (vector<Entity>::const_iterator entity = entityList.begin(), end = entityList.end(); entity != end; ++entity) {
        result.append(text, pos, entity->start - pos);
        result.append(entity->html);
        pos = entity->end;
    }
    result.append(text, pos);
    return result;
}

vector<set<Entity>* > createEntriesList(string text) {
    default_random_engine generator;
    uniform_int_distribution<int> distribution(0, 9);
    uniform_int_distribution<int> distribution2 = uniform_int_distribution<int>(0, (int) (text.length() - 1));
    auto r = bind(distribution, generator);
    auto r2 = bind(distribution2, generator);
    auto *entityList = new vector<set<Entity>* >();
    for (int i = 0; i < 1000; i++) {
        auto entitySet = new set<Entity>();
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
            string html = "";
            for (int k = 0; k < length; k++) {
                html.append("XX");
            }
            entitySet->insert(*new Entity(start, end, html));
        }
        entityList->push_back(entitySet);
    }
    return *entityList;
}

long currentTimeMillis() {
    using namespace std::chrono;
    milliseconds ms = duration_cast<milliseconds>(system_clock::now().time_since_epoch());
    return ms.count();
}

void bench() {
    string text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    auto entityList = createEntriesList(text);
    
    {
        for (int j = 0; j < 5; j++) {
            long start = currentTimeMillis();
            for (int i = 0; i < 1000000; i++) {
                set<Entity>* entitySet = entityList[i % 1000];
                render(text, entitySet);
            }
            cout << (currentTimeMillis() - start) << " ns/op\n";
        }
    }

}

int main(int argc, const char * argv[]) {
    
    auto *entitySet = new set<Entity>();
    entitySet->insert(*new Entity(25, 32, "<#mobile>"));
    entitySet->insert(*new Entity(33, 42, "<#startups>"));
    entitySet->insert(*new Entity(46, 51, "<#OF12>"));
    entitySet->insert(*new Entity(82, 102, "<http://t.co/HtzEMgAC>"));
    entitySet->insert(*new Entity(103, 110, "<@TiEcon>"));
    entitySet->insert(*new Entity(111, 127, "<@sv_entrepreneur>"));
    entitySet->insert(*new Entity(128, 132, "<@500>"));
    
    string text = "Attend to hear 6 stellar #mobile #startups at #OF12 Entrepreneur Idol show 2day,  http://t.co/HtzEMgAC @TiEcon @sv_entrepreneur @500!";
    string test = "Attend to hear 6 stellar <#mobile> <#startups> at <#OF12> Entrepreneur Idol show 2day,  <http://t.co/HtzEMgAC> <@TiEcon> <@sv_entrepreneur> <@500>!";

    string result = render(text, entitySet);
    cout << result << ": " << (test == result) << "\n";
    
    bench();
    
    return 0;
}
