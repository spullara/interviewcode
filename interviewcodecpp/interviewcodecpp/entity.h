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

    struct PtrComparator {
        bool operator()(Entity const* left, Entity const* right) const {
            return left->start < right->start;
        }
    };

    struct UniquePtrComparator {
        bool operator()(std::unique_ptr<Entity> const& left, std::unique_ptr<Entity> const& right) const {
            return left->start == right->start &&
                left->end == right->end &&
                left->html == right->html;
        }
    };

    Entity(int, int, u32string&&);
};

Entity::Entity(int start, int end, u32string&& html) {
    this->start = start;
    this->end = end;
    this->html = std::move(html);
}

template <>
struct std::hash<unique_ptr<Entity>>
{
    std::size_t operator()(unique_ptr<Entity> const& k) const
    {
        using std::size_t;
        using std::hash;
        using std::string;

        // Compute individual hash values for first,
        // second and third and combine them using XOR
        // and bit shifting:
        return ((_int_hash(k->start) ^ (_int_hash(k->end) << 1)) >> 1) ^ (_u32string_hash(k->html) << 1);
    }

private:
    std::hash<int> _int_hash;
    std::hash<u32string> _u32string_hash;
};

#endif /* entity_h */
