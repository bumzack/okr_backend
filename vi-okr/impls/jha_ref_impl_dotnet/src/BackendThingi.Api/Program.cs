global using FastEndpoints;
using BackendThingi.Api;
using BackendThingi.Api.Services;

// Create a new SlimApplicationBuilder.
var builder = WebApplication.CreateSlimBuilder(args);

// Load the configuration from the appsettings.json file.
builder.Services.Configure<BackendThingiOptions>(builder.Configuration.GetSection(BackendThingiOptions.BackendThingi));

// Add the FastEndpoints services to the application.
builder.Services.AddFastEndpoints();

// Add the article services to the application.
builder.Services.AddScoped<ArticleService>();

// Build the application.
var app = builder.Build();

// Configure FastEndpoints
app.UseFastEndpoints(config =>
{
    config.Endpoints.RoutePrefix = "api";

    config.Versioning.DefaultVersion = 1;
    config.Versioning.Prefix = "v";
    config.Versioning.PrependToRoute = true;
});

// Run the application.
app.Run();