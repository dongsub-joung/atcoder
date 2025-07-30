#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main() {
    int n, start, end;
    char *str;

    scanf("%d %d %d", &n, &start, &end);

    str = (char *)malloc(n * sizeof(char));
    scanf("%s", str);

    bool pass[n];
    for (int i= start-1, j=0; i<end-1; i++) {
        bool maru= (str[i] == 'o') ? true : false;
        pass[j++]= maru;
    }

    int anwser= 0;
    for (int i=0; i<sizeof(pass); i++) {
        anwser+= pass[i];
    }

    (end-start == anwser) ? printf("Yes") : printf("No");

    return 0;   
}