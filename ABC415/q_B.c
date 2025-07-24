#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]){
    int size_of_intput = 100;
    char *s= (char *) malloc(size_of_intput);
    char *result= (char *) malloc(size_of_intput);

    scanf("%s", s);    
    if (s == NULL) {
        printf("Allocation Failed");
        exit(0);
    }

    int j=0;
    for (int i = 0; i <= size_of_intput; i++){
        if (s[i] == '#'){
            result[j++]= i+1;
        }
    }

    for (int i = 0, j=1; i <= sizeof(result)+1; i=i+2, j=j+2){
        printf("%d,%d\n", result[i], result[j]);
    }

    // free(s);
    // free(result);

    return 0;
}