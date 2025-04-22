import java.util.Scanner

fun main() {
    val sc = Scanner(System.`in`)
    var line= sc.nextLine()

    if (line.contains("ACE")
        || line.contains("BDF")
        || line.contains("CEG")
        || line.contains("DFA")
        || line.contains("EGB")
        || line.contains("FAC")
        || line.contains("GBD"))
        println("Yes")
    else
        println("No")
}