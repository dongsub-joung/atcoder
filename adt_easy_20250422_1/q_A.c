#include <stdio.h>
#include <string.h>
#include <stdbool.h> 

void trim(char *s) {
  
    // Two pointers initially at the beginning
  int i = 0, j = 0;

  // Skip leading spaces
  while (s[i] == ' ') i++; 

  // Shift the characters of string to remove
    // leading spaces
  while (s[j++] == s[i++]);
}


int main() {
    char str[10];
    bool flag[]= {false, false};
    fgets(str, strlen(str), stdin);
    str[strcspn(str, "\n")] = 0;
    
    trim(str);

    int i=0;
    if (*str == 'A'){
        if (str[i+1] == 'C' && str[i+2] == 'E') {
            printf("%s" ,"Yes");
        }else {
            printf("%s" ,"No");
        }
    }
    else if(*str == 'B'){
        if (str[i+1] == 'D' && str[i+2] == 'F') {
            printf("%s" ,"Yes");
        }else{
            printf("%s" ,"No");
        }
    }
    else if (*str == 'C') {
        if (str[i+1] == 'E' && str[i+2] == 'G') {
            printf("%s" ,"Yes");
        }else {
            printf("%s" ,"No");
        }
    }
    else if (*str == 'D') {
        if (str[i+1] == 'F' && str[i+2] == 'A') {
            printf("%s" ,"Yes");
        }else {
            printf("%s" ,"No");
        }
    }
    else if (*str == 'E') {
        if (str[i+1] == 'G' && str[i+2] == 'B') {
            printf("%s" ,"Yes");
        }else{
            printf("%s" ,"No");
        }
    }
    else if (*str == 'F') {
        if (str[i+1] == 'A' && str[i+2] == 'C') {
            printf("%s" ,"Yes");
        }else {
            printf("%s" ,"No");
        }
    }
    else if (*str == 'G') {
        if (str[i+1] == 'B' && str[i+2] == 'D') {
            printf("%s" ,"Yes");
        }else {
            printf("%s" ,"No");
        }
    }else{
        printf("%s" ,"No");
    }

    return 0;
}