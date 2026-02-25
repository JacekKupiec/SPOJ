// HMRO.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#define _CRT_SECURE_NO_WARNINGS

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unordered_map>

using namespace std;

struct Node {
    Node* parent;
    uint32_t rank;
};

Node* MakeSet() {
    Node* n = new Node;

    n->parent = n;
    n->rank = 0;

    return n;
}

Node* FindSet(Node* n) {
    if (n->parent != n) {
        n->parent = FindSet(n->parent);
    }

    return n->parent;
}

void Link(Node* left_node, Node* right_node) {
    if (left_node->rank > right_node->rank)
        right_node->parent = left_node;
    else {
        left_node->parent = right_node;

        if (left_node->rank == right_node->rank)
            right_node->rank++;
    }
}

void Union(Node* left_node, Node* right_node) {
    Link(FindSet(left_node), FindSet(right_node));
}

int32_t ConvertToIntegerCode(char* mroCode) {
    return *((int32_t*)mroCode);
}

int main()
{
    int numberOfTests;

    scanf("%d", &numberOfTests);

    while (numberOfTests-- > 0)
    {
        int numberOfBoys;
        unordered_map<int32_t, Node*> mros;
        unordered_map<uint64_t, Node*> recruits;
        unordered_map<Node*, char*> nameOfMros;

        scanf("%d", &numberOfBoys);

        for (int i = 0; i < numberOfBoys; i++) {
            uint64_t pesel;
            char mroCode[5];

            scanf("%llu %s", &pesel, mroCode);

            Node* recruit = MakeSet();
            recruits.insert(make_pair(pesel, recruit));

            int32_t mroCodeConverted = ConvertToIntegerCode(mroCode);
            auto result = mros.find(mroCodeConverted);

            if (result == mros.end()) {
                Node* mroNode = MakeSet();

                Union(recruit, mroNode);
                mros.insert(make_pair(mroCodeConverted, mroNode));

                char *nameOfMro = new char[5];

                strcpy(nameOfMro, mroCode);
                nameOfMros.insert(make_pair(mroNode, nameOfMro));
            }
            else {
                Union(recruit, result->second);
            }
        }

        int numberOfClosedMro;

        scanf("%d", &numberOfClosedMro);

        for (int i = 0; i < numberOfClosedMro; i++) {
            char closedMroName[5], targetMroName[5];

            scanf("%s %s", closedMroName, targetMroName);

            auto closedMroNode = mros[ConvertToIntegerCode(closedMroName)];
            auto closedMroNodeParent = FindSet(closedMroNode);

            auto targetMroNode = mros[ConvertToIntegerCode(targetMroName)];
            auto targetMroNodeParent = FindSet(targetMroNode);

            // consider releasing the memory that was taken to store names
            delete[] nameOfMros[closedMroNodeParent];
            delete[] nameOfMros[targetMroNodeParent];
            nameOfMros.erase(targetMroNodeParent);
            nameOfMros.erase(closedMroNodeParent);

            Union(closedMroNodeParent, targetMroNodeParent);

            auto finalNode = FindSet(targetMroNodeParent);
            char* nameOfMro = new char[5];

            strcpy(nameOfMro, targetMroName);
            nameOfMros.insert(make_pair(finalNode, nameOfMro));
        }

        int numberOfRequests;

        scanf("%d", &numberOfRequests);

        for (int i = 0; i < numberOfRequests; i++)
        {
            uint64_t pesel;

            scanf("%llu", &pesel);

            Node* mroParentNode = FindSet(recruits[pesel]);
            
            printf("%llu %s\n", pesel, nameOfMros[mroParentNode]);
        }

        putchar('\n');
    }
}
