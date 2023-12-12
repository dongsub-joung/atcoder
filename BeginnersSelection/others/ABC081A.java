import java.util.Scanner;

public class ABC081A {
    public static void main(String[] args) {
        Scanner sc= new Scanner(System.in);
        var input= sc.next();
        String str= String.valueOf(input);
        char[] strs= str.toCharArray();
        int cnt= 0;
        for(int i=0; i<str.length(); i++){
            if(strs[i] == '1'){
                cnt++;
            }
        }
        System.out.println(cnt);
    }
}