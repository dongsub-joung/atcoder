import java.util.ArrayList;
import java.util.Collections;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        ArrayList nums=new ArrayList();
        Scanner sc=new Scanner(System.in);

        var N=sc.nextInt();
        for(int i=1;i<=N;i++){
            nums.add(sc.nextInt());
        }
        Integer all_max= (Integer) Collections.max(nums);
        if(nums.get(0) ==all_max){
            System.out.println(0);
            System.exit(0);
        }else{
            int temp= all_max.intValue();
            var answer= temp - nums.get(0) + 1;
            System.out.println(answer);
    }
}