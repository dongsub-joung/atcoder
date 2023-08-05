import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc= new Scanner(System.in);
        var a= sc.nextInt();
        var b= sc.nextInt();

        if (a*b % 2 == 0){
            System.out.println("Even");
        }else{
            System.out.println("Odd");
        }
    }
}