// CPP_Experiments.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#define _CRT_SECURE_NO_WARNINGS

#include <stdint.h>
#include <stdio.h>
#include <time.h>

const uint64_t BigDivider = 4000000007;
const uint64_t TimestampTreshold = BigDivider / 2;
const uint64_t IndexToSolveEquation = (BigDivider + 1) / 4;

uint64_t PowMod(uint64_t x, uint64_t y, uint64_t p)
{
    uint64_t result = 1;

    if (y <= 0)
    {
        return 0;
    }

    do
    {
        if ((y & 1) != 0)
        {
            result = (result * x) % p;
        }

        x = (x * x) % p;
        y >>= 1;
    } while (y > 0);

    return result;
}

int main()
{
    uint64_t encryptedTimestamp;

    scanf("%llu", &encryptedTimestamp);

    /* Nie ma sprawdzenia, czy istnieje rozwiązanie tego równania, na potrzeby zadania nie trzeba tego sprawdzać */

    uint64_t decryptedTimestsmp = PowMod(encryptedTimestamp, IndexToSolveEquation, BigDivider);

    if (decryptedTimestsmp > TimestampTreshold)
    {
        decryptedTimestsmp = BigDivider - decryptedTimestsmp;
    }

    struct tm* info = gmtime((time_t*)&decryptedTimestsmp);

    char output[80];
    strftime(output, sizeof(output), "%a %b %d %T %Y", info);

    printf("%s", output);

    return 0;

}
