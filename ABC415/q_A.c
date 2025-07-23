#include <stdio.h>
#include <stdbool.h> 
#include <stdlib.h>


bool containsElement(int arr[], int size, int target) {
    for (int i = 0; i < size; i++) {
        if (arr[i] == target) {
            return true; // Element found
        }
    }
    return false; // Element not found
}

int main(int argc, char *argv[]) {
    int n=0; int x=0;
    int arr[n];

    scanf("%d", &n);
    
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    scanf("%d", &x);
    
    char *result;
    int num_chars = 5;
    result= (char *)malloc(num_chars * sizeof(char)); 

    result= containsElement(arr, n, x) ? "Yes" : "No";
    
    printf("%s", result);

    return 0;
}