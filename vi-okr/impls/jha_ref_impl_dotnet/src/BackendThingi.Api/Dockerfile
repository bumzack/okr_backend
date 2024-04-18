FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["src/BackendThingi.Api/BackendThingi.Api.csproj", "src/BackendThingi.Api/"]
RUN dotnet restore "src/BackendThingi.Api/BackendThingi.Api.csproj"
COPY . .
WORKDIR "/src/src/BackendThingi.Api"
RUN dotnet build "BackendThingi.Api.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "BackendThingi.Api.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

RUN apt-get update && apt-get install -y curl

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "BackendThingi.Api.dll"]
