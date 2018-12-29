//
//  entity.h
//  interviewcodecpp
//
//  Created by Sam Pullara on 1/13/17.
//  Copyright © 2017 Sam Pullara. All rights reserved.
//

#ifndef entity_h
#define entity_h

#include <string>
#include <functional>

using namespace std;

class Entity : less<Entity> {
public:
    int start;
    int end;
    u32string html;
    
    Entity(int, int, u32string&&);
    bool operator<(const Entity &e) const {
        return start < e.start;
    }};

Entity::Entity(int start, int end, u32string&& html) {
    this->start = start;
    this->end = end;
    this->html = std::move(html);
}


#endif /* entity_h */
