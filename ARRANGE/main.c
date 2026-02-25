#include <stdlib.h>
#include <stdio.h>

int cmp(const void *a, const void *b) {
    if (*(int*)a == 1) return -1;
    if (*(int*)b == 1) return 1;

    return *(int*)b - *(int*)a;
}

int main(int argc, char **argv) {
    int tab[100000], n, m, j;

    scanf("%d", &n);

    for (;n > 0; n--) {
        scanf("%d", &m);

        for (j = 0; j < m; j++)
            scanf("%d", tab + j);

        qsort(tab, m, sizeof(int), cmp);

        if (m == 2 || m > 2 && tab[m - 3] == 1)
            if (tab[m - 2] == 3 && tab[m - 1] == 2) {
                tab[m - 2] = 2;
                tab[m - 1] = 3;
            }

        for (j = 0 ; j < m; j++)
            printf("%d ", tab[j]);
        putchar('\n');
    }

    return EXIT_SUCCESS;
}