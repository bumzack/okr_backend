FROM eclipse-temurin:21.0.2_13-jdk as build

# copy build support
COPY importbench /server
# build
WORKDIR /server
RUN ["./gradlew", "build"]


FROM eclipse-temurin:21.0.2_13-jre

EXPOSE 8080
VOLUME /data

WORKDIR /server
RUN useradd -ms /bin/bash app
RUN chown -R app:app /server
USER app

COPY --chown=app:app --from=build /server/build/libs/fat.jar /server/server.jar

CMD ["java", "-jar", "/server/server.jar", "-P:media.dir=/data"]
