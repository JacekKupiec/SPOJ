#include <stdio.h>
#include <math.h>
#include <string.h>

#define MNUMBER 46340

const unsigned MAX_NUMBER = MNUMBER;
const unsigned LAST_PRECOMPUTED_PRIME = 46337;

#define CHECK_BIT(t, i) ((t)[(i) >> 4] & bit_set[((i) & 15) >> 1])
#define CLEAR_BIT(t, i) ((t)[(i) >> 4] &= bit_clear[((i) & 15) >> 1])

unsigned short primes[4792]; /* Tabela zawiera wszystkie liczby pierwsze, nawet 2 */
unsigned char tab[MNUMBER / 16 + 1]; /* Nie chcę parzystych i 8 liczb moge pomieścic w bajcie */
unsigned char segment[1000001 / 16 + 2]; /* Maksymalnie tyle potrzebuję elementów żeby przeszukać zadany przedział */
unsigned char bit_clear[8] = { 0xFE, 0xFD, 0xFB, 0xF7, 0xEF, 0xDF, 0xBF, 0x7F };
unsigned char bit_set[8] = { 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80 };

void remove_multiples_of(unsigned char* t, unsigned range_max, unsigned number) {
    unsigned j, step;

    for (j = number*number, step = number << 1; j <= range_max; j += step)
        t[j >> 4] &= bit_clear[(j & 15) >> 1];
}


void initialize() {
    unsigned i, next = 0;
    const unsigned N = (unsigned)sqrt((double)MAX_NUMBER), 
        lookup[2] = { 2, 4 };

    memset(tab, 0xFF, sizeof(tab) / sizeof(tab[0]));
    tab[0] = 0x6e; /* Inicjalizuje pierwszy bajt */
    remove_multiples_of(tab, MAX_NUMBER, 3);

    i = 5;
    while (i <= N) {
        if (CHECK_BIT(tab, i)) /* Jeśli liczba jest pierwsza to w ogóle wchodzimy */
            remove_multiples_of(tab, MAX_NUMBER, i);

        i += lookup[next];
        next = !next;
    }
}


void rewrite_primes(unsigned char *sieve, unsigned short *p, unsigned limit) {
    unsigned i, next = 0, j;
    const unsigned  lookup[2] = { 2, 4 };

    if (limit > 2) p[0] = 2;

    if (limit > 3) p[1] = 3;

    for (i = 5, j = 2; i <= limit; i += lookup[next], next = !next)
        if (CHECK_BIT(sieve, i))
            p[j++] = i;
}


/* Policzyć ile mam nieparzystych i jak ustawię wszystkie bity, to szukac tak długo,
dopóki będę miał choć jeden ustawiony bit. Wykorzystać numbers_in_interval.
Wykorzystać liczby pierwsze tylko z zakresu sqrt(n) */  
void find_primes_in_segment(unsigned char *segment, unsigned short *primes, unsigned limit, unsigned interval, unsigned m, unsigned n) {
    unsigned i = 1;

    /* Jeśli mam parzystą liczbe elementów to liczb nieparzystych jest połowa. Jeśli mam nieparzystą liczbe elementów to jeśli 
    pierwsza jest nie parzysta to nieparzystych jest o 1 więcej. Nieparzystość_liczby_elementów * nieparzysty_pierwszy_element,
    jeśli daje 1 to dadaję do liczby liczb nieparzystych. Robię to bez instrukcji if */
    interval = (interval >> 1) + ((m & 1) & (interval & 1)); 

    /* Nie wolno wykreślać samych liczb pierwszych. Nie można dopuścić do tego, 
    aby w przypadku sprawdzania wuielokrotności 3 wykreślono samo 3. */

    while (primes[i] <= limit && interval && i < 4792) { /* Wykreślam wszystko z danego przedziału*/
        unsigned start = ((m + primes[i] - 1) / primes[i]) * primes[i], 
            step = primes[i] << 1,
            sqr = primes[i] * primes[i];

        if (!(start & 1)) start += primes[i]; /* start zawsze jest wielokrotnością nieparzystą */
        if (sqr > start) start = sqr;

        for (; start <= n; start += step)
            if (CHECK_BIT(segment, start - m)) {
                CLEAR_BIT(segment, start - m);
                --interval;
            }

        ++i;
    }
}


unsigned print_small_primes(unsigned m) {
    switch (m)
    {
        case 1:
        case 2:
            puts("2\n3");
            return 5;
        case 3:
            puts("3");
            return 5;
        default: break;
    }

    return (m & 1) ? m : m + 1;
}


int main() {
    unsigned t, n, m;

    scanf("%u", &t);

    if (t > 0) {
        initialize(); /* Inicjalizuje wyszukiwanie z przedziału 1..sqrt(N) */
        rewrite_primes(tab, primes, MAX_NUMBER); /* Wszystkie liczby pierwsze mam w tej tabeli */
    }

    while(t-- > 0) { 
        scanf("%u %u", &m, &n);

        if (1 || n > LAST_PRECOMPUTED_PRIME) {
            unsigned numbers_in_interval, limit, i = 1, odd = 0, /* "i" jest 1 bo zaczynam od 3, a nie od 2 */
                next[2] = { 2, 4 }, 
                align[6] = { 1, 0, 3, 2, 1, 0 };

            numbers_in_interval = n - m + 1;
            memset(segment, 0xFF, (numbers_in_interval >> 4) + (numbers_in_interval & 1) + 1); /* Wyzerowuję tę część tablicy, która będzie potrzebna w segmencie */
            find_primes_in_segment(segment, primes, (unsigned)sqrt((double)n), numbers_in_interval, m, n);

            /* Inicjalizacja zmiennych przed wydrukowaniem */
            i = print_small_primes(m);
            i += align[i % 6];

            if (i % 6 == 1) {
                next[0] = 4;
                next[1] = 2;
            }
            for(; i <= n; i += next[odd], odd = !odd) 
                if (CHECK_BIT(segment, i - m))
                    printf("%u\n", i);

       } else { /* przypadek, w którym mam już wszystko policzone, wystarczy wypisać */
            unsigned i;

            for (i = 0; primes[i] <= n; i++)
                if (primes[i] >= m)
                    printf("%u\n", primes[i]);
        }

        putchar('\n');
    }

    return 0;
}