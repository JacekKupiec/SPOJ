#define _CRT_SECURE_NO_WARNINGS

#define LENGTH_OF_CITY_NAME 10

#include <vector>
#include <unordered_map>
#include <set>

using namespace std;

struct Neighbour {
    int destination, cost;
};

struct VerticleDistance {
    int verticleNumber, distance;

    VerticleDistance(int verticle, int distance) {
        this->verticleNumber = verticle;
        this->distance = distance;
    }

    bool operator < (const VerticleDistance& right) const {
        return this->distance < right.distance
            || (this->distance == right.distance && this->verticleNumber < right.verticleNumber);
    }
};

bool VerticlesCompare(VerticleDistance first, VerticleDistance second) {
    return first.distance < second.distance;
}

int Dijkstry(Neighbour** graph, int numberOfCities, int source, int destination)
{
    if (source == destination) return 0;

    vector<int> verticlesDistances(numberOfCities);
    set<VerticleDistance> verticlesDistancesQueue;

    for (int cityIdx = 0; cityIdx < numberOfCities; cityIdx++)
    {
        verticlesDistancesQueue.emplace(cityIdx, INT32_MAX);
        verticlesDistances[cityIdx] = INT32_MAX;
    }

    const VerticleDistance sourceToDelete(source, INT32_MAX);

    verticlesDistancesQueue.erase(sourceToDelete);
    verticlesDistancesQueue.emplace(source, 0);
    verticlesDistances[source] = 0;

    auto currentVertex = verticlesDistancesQueue.cbegin();

    do {
        // Calculate new distances
        for (int neighbourIdx = 0; graph[currentVertex->verticleNumber][neighbourIdx].destination != -1; neighbourIdx++)
        {
            auto newDistance = currentVertex->distance + graph[currentVertex->verticleNumber][neighbourIdx].cost;
            auto neighbour = graph[currentVertex->verticleNumber][neighbourIdx].destination;
            auto currentDistance = verticlesDistances[neighbour];

            if (newDistance < currentDistance) {
                VerticleDistance toDelete(neighbour, currentDistance);

                verticlesDistancesQueue.erase(toDelete);
                verticlesDistancesQueue.emplace(neighbour, newDistance);
                verticlesDistances[neighbour] = newDistance;
            }
        }

        verticlesDistancesQueue.erase(currentVertex);
        currentVertex = verticlesDistancesQueue.cbegin();

    } while (currentVertex->verticleNumber != destination && !verticlesDistancesQueue.empty());

    return verticlesDistances[destination];
}

int main()
{
    int testNum;

    scanf("%d", &testNum);

    while (testNum--) {
        int numberOfCities;
        unordered_map<string, int> citiesNames;

        scanf("%d", &numberOfCities);
        Neighbour **graph = new Neighbour* [numberOfCities];

        for (int cityIdx = 0; cityIdx < numberOfCities; cityIdx++)
        {
            char cityName[LENGTH_OF_CITY_NAME + 1];
            int numberOfNeighbours;

            scanf("%s\n%d", cityName, &numberOfNeighbours);

            string cityNameAsString(cityName);

            citiesNames.insert(make_pair(cityNameAsString, cityIdx));
            graph[cityIdx] = new Neighbour[numberOfNeighbours + 1];
            graph[cityIdx][numberOfNeighbours].destination = -1;

            for (int neighboursIdx = 0; neighboursIdx < numberOfNeighbours; neighboursIdx++)
            {
                int neighbour, cost;

                scanf("%d %d", &neighbour, &cost);

                graph[cityIdx][neighboursIdx].destination = neighbour - 1;
                graph[cityIdx][neighboursIdx].cost = cost;
            }
        }

        int pathsToFind;

        scanf("%d", &pathsToFind);

        for (int pathsIdx = 0; pathsIdx < pathsToFind; pathsIdx++)
        {
            char source[LENGTH_OF_CITY_NAME + 1], destination[LENGTH_OF_CITY_NAME +  1];

            scanf("%s %s", source, destination);

            string sourceName(source);
            string destinationName(destination);

            auto sourceIdx = citiesNames.find(sourceName);
            auto destinationIdx = citiesNames.find(destinationName);

            int distance = Dijkstry(graph, numberOfCities, sourceIdx->second, destinationIdx->second);

            printf("%d\n", distance);
        }

        for (int cityIdx = 0; cityIdx < numberOfCities; cityIdx++) delete[] graph[cityIdx];
        delete[] graph;
    }
}
