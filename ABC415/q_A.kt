fun contain(n: Int, x: Int, arr: List<Int>): Boolean{
    for(i in 0..n-1){
        if (arr[i] == x){
            return true
        }
    }

    return false
}
fun main() {
    val n= readln().toInt();
    val arr= readln().split(' ').map { it.toInt() }
    val x= readln().toInt()

    val result= if(contain(n, x, arr)) "Yes" else "No"

    println(result)
}
