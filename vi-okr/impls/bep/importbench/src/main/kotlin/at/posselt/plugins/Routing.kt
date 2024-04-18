package at.posselt.plugins

import at.posselt.api.ImportRequest
import at.posselt.api.SysInfo
import at.posselt.api.filesInDirectory
import at.posselt.api.serialize
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import java.nio.file.Paths

fun Application.configureRouting() {
    val dataDir = environment.config.propertyOrNull("media.dir")!!.getString()
    println(dataDir)
    routing {
        get("/api/v1/sysinfo") {
            call.respond(
                SysInfo(
                    author = "Bernhard Posselt",
                    language = "Kotlin",
                    framework = "KTOR",
                    multithreaded = false,
                    version = "0.0.1",
                    comment = "KTOR + Jackson Serialization"
                )
            )
        }
        post("/api/v1/articles/import") {
            val args = call.receive<ImportRequest>()
            val path = Paths.get(dataDir)
            val files = filesInDirectory(path)
            call.respondOutputStream {
                serialize(args.returnItems, files, this)
            }
        }
    }
}
