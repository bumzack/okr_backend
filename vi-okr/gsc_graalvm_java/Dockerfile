FROM ghcr.io/graalvm/jdk-community:21 as build

RUN mkdir -p /usr/share/okr
# RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN microdnf install findutils

COPY gradlew /usr/share/okr/gradlew
COPY gradle /usr/share/okr/gradle
COPY build.gradle.kts /usr/share/okr/build.gradle.kts
COPY settings.gradle.kts /usr/share/okr/settings.gradle.kts
COPY src /usr/share/okr/src

# build
WORKDIR /usr/share/okr

RUN ["./gradlew", "assemble"]
RUN ["/bin/bash", "-c", "cp /usr/share/okr/build/libs/graalvm-java-21-0.0.1-SNAPSHOT.jar /usr/share/okr/graalvm-java-21.jar"]

FROM ghcr.io/graalvm/jdk-community:21

EXPOSE 2323

WORKDIR /usr/share/okr

COPY  --from=build /usr/share/okr/graalvm-java-21.jar /usr/share/okr/graalvm-java-21.jar
