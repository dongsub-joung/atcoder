// Online Java Compiler
// Use this editor to write, compile and run your Java code online
class Soltuion{
    public static void init(int n, String[] votes){
        int cnt= 0;
        for(var e : votes){
            if (e.equals("For")){
                cnt+=1;
            }
        }
        
        if(n == 1 & cnt >= 1){
            System.out.println("Yes");
        }
        else if((n/2) < cnt){
            System.out.println("Yes");        
        }else{
            System.out.println("No");
        }
    }
}

class Main {
    public static void main(String[] args) {
        int n= 3;
        String[] vote= {"For", "Against", "For"};
        Soltuion.init(n, vote);
        
        int n2= 5;
        String[] vote2= {"Against", "Against", "For", "Against", "For"};
        Soltuion.init(n2, vote2);
        
        int n3= 1;
        String[] vote3= {"For"};
        Soltuion.init(n3, vote3);
    }
}
