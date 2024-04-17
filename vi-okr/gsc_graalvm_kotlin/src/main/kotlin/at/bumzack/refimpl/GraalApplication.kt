package at.bumzack.refimpl

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class GraalApplication

fun main(args: Array<String>) {
    runApplication<GraalApplication>(*args)
}
