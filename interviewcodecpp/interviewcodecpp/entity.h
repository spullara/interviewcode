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
    }
    bool operator==(const Entity &e) const {
        return start == e.start && end == e.end && html == e.html;
    }
};

Entity::Entity(int start, int end, u32string&& html) {
    this->start = start;
    this->end = end;
    this->html = std::move(html);
}

template <>
struct std::hash<Entity>
{
    std::size_t operator()(const Entity& k) const
    {
        using std::size_t;
        using std::hash;
        using std::string;
        
        // Compute individual hash values for first,
        // second and third and combine them using XOR
        // and bit shifting:
        
        return ((hash<int>()(k.start)
                 ^ (hash<int>()(k.end) << 1)) >> 1)
        ^ (hash<u32string>()(k.html) << 1);
    }
};

#endif /* entity_h */
