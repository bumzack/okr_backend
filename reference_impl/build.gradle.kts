plugins {
    java
    id("org.springframework.boot") version "3.2.2"
    id("io.spring.dependency-management") version "1.1.4"
}

group = "at.bumzack"
version = "0.0.1-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_21
}

tasks.getByName<Jar>("jar") {
    enabled = false
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-web:3.1.0")
    // implementation("org.springframework.boot:spring-boot-starter-data-jpa:3.0.6")
    // runtimeOnly("org.postgresql:postgresql:42.5.4")
}

tasks.withType<Test> {
    useJUnitPlatform()
}
