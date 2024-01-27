FROM eclipse-temurin:21-jdk as build

RUN mkdir -p /usr/share/okr
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl

COPY src /usr/share/okr/src
COPY gradlew /usr/share/okr/gradlew
COPY gradle /usr/share/okr/gradle
COPY build.gradle.kts /usr/share/okr/build.gradle.kts
COPY settings.gradle.kts /usr/share/okr/settings.gradle.kts

# build
WORKDIR /usr/share/okr

RUN ["./gradlew", "assemble"]
RUN ["/bin/bash", "-c", "cp /usr/share/okr/build/libs/java21-reference-impl-*.jar /usr/share/okr/java21-reference-impl.jar"]


# TODO find a jre
FROM eclipse-temurin:21-jdk

EXPOSE 2323

WORKDIR /usr/share/okr

COPY --from=build /usr/share/okr/java21-reference-impl.jar /usr/share/okr/java21-reference-impl.jar
