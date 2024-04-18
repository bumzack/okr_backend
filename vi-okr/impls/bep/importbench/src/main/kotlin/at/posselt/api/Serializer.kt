package at.posselt.api

import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.json.JsonMapper
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import java.io.OutputStream
import java.nio.file.Path

fun serialize(returnItems: Boolean, files: List<Path>, outputStream: OutputStream) {
    val mapper = JsonMapper.builder()
        .addModule(JavaTimeModule())
        .build()
    mapper.createGenerator(outputStream).use { generator ->
        var processed = 0
        var written = 0
        generator.createObject {
            if (returnItems) {
                createArray("articles") {
                    forEachArticle(files) {
                        writeObject(it.cheapest)
                        processed += it.processedRows
                        written += 1
                    }
                }
            } else {
                forEachArticle(files) {
                    processed += it.processedRows
                    written += 1
                }
            }
            writeNumberField("linesProcessed", processed)
            writeNumberField("dbRowsWritten", written)
        }
    }
}

internal fun JsonGenerator.createArray(fieldName: String? = null, lambda: JsonGenerator.() -> Unit) {
    if (fieldName != null) {
        writeFieldName(fieldName)
    }
    writeStartArray()
    lambda()
    writeEndArray()
}

internal fun JsonGenerator.createObject(fieldName: String? = null, lambda: JsonGenerator.() -> Unit) {
    if (fieldName != null) {
        writeFieldName(fieldName)
    }
    writeStartObject()
    lambda()
    writeEndObject()
}