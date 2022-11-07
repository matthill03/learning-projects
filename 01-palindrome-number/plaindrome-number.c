#include <stdio.h>

int reverse_number(int num) {
  int remainder, reverse = 0;

  while(num != 0) {
    remainder = num % 10; 
    reverse = reverse * 10 + remainder;
    num /= 10;
  }

  return reverse;
}

int main() {
  
  int input = 0;

  printf("Enter a number: ");
  scanf("%d",&input);

  int reverse = reverse_number(input);

  if (input == reverse) {
    printf("This is a plaindrome number\n");
  }
  else {
    printf("This is not a plaindrome number\n");
  }
  

  return 0;
}
