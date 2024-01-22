plugins {
    java
    id("org.springframework.boot") version "3.2.1"
    id("io.spring.dependency-management") version "1.1.4"
    id ("org.graalvm.buildtools.native")  version "0.9.24"
}

group = "at.bumzack"
version = "0.0.1-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_21
}

graalvmNative {
    binaries {
        named("main") {
            imageName.set("graal-ref-impl-app")
            mainClass.set("at.bumzack.reference.impl.ReferenceImplApplication")
            buildArgs.add("-O4")
            javaLauncher.set(javaToolchains.launcherFor {
                languageVersion.set(JavaLanguageVersion.of(21))
                vendor.set(JvmVendorSpec.matching("Oracle Corporation"))
            })
        }

        named("test") {
            buildArgs.add("-O0")
        }
    }
    binaries.all {
        buildArgs.add("--verbose")
        buildArgs.add("--enable-preview")

    }
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter-data-jpa:3.0.6")
    implementation("org.springframework.boot:spring-boot-starter-web:3.1.0")
    runtimeOnly("org.postgresql:postgresql:42.5.4")
    testImplementation("org.springframework.boot:spring-boot-starter-test:3.1.0")
}

tasks.withType<Test> {
    useJUnitPlatform()
}
