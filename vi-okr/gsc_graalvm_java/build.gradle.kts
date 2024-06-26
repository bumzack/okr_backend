plugins {
    java
    id("org.springframework.boot") version "3.2.2"
    id("io.spring.dependency-management") version "1.1.4"
    id ("org.graalvm.buildtools.native")  version "0.9.28"
}

group = "at.bumzack"
version = "0.0.1-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_21
}

//tasks.getByName<Jar>("jar") {
//    enabled = false
//}

repositories {
    mavenCentral()
    maven { url = uri("https://repo.spring.io/milestone") }
    maven { url = uri("https://repo.spring.io/snapshot") }
}

graalvmNative {
    binaries {
        named("main") {
            imageName.set("graal-java-app")
//            mainClass.set("at.bumzack.reference.impl.GraalVmJavaApplication")
            buildArgs.add("-O4")
            buildArgs.add("-march=native")
            javaLauncher.set(javaToolchains.launcherFor {
                languageVersion.set(JavaLanguageVersion.of(21))
       //         vendor.set(JvmVendorSpec.matching("Oracle Corporation"))
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
    implementation("org.springframework.boot:spring-boot-starter-web:3.2.2")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin")
    implementation("org.jetbrains.kotlin:kotlin-reflect")

    // https://github.com/spring-projects/spring-data-commons/issues/3025
    // sooner or later this can be removed
    implementation("org.springframework.data:spring-data-commons:3.2.x-3025-SNAPSHOT")
}

tasks.withType<Test> {
    useJUnitPlatform()
}


//tasks.withType<KotlinCompile> {
//    kotlinOptions {
//        freeCompilerArgs += "-Xjsr305=strict"
//        jvmTarget = "21"
//    }
//}
