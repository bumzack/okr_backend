plugins {
    java
    id("org.springframework.boot") version "3.2.1"
    id("io.spring.dependency-management") version "1.1.4"
    id("org.graalvm.buildtools.native") version "0.9.28"

    id("org.openrewrite.rewrite") version("6.6.4")
}

rewrite {
    activeRecipe("org.openrewrite.java.logging.logback.Log4jToLogback")
}



group = "at.bumzack"
version = "0.0.1-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_21
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-data-jpa:3.0.6")
    implementation("org.springframework.boot:spring-boot-starter-web:3.1.0")
    runtimeOnly("org.postgresql:postgresql:42.5.4")
    testImplementation("org.springframework.boot:spring-boot-starter-test:3.1.0")

   //  implementation("org.springframework.experimental:spring-native:0.12.0")
    compileOnly("org.graalvm.nativeimage:svm:23.1.1")

    rewrite("org.openrewrite.recipe:rewrite-logging-frameworks:2.4.0")
}

tasks.withType<Test> {
    useJUnitPlatform()
}
