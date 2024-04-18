namespace BackendThingi.Api.Features.Public.SysInfo.GetSysInfo;

/// <summary>
/// Represents an endpoint for retrieving system information.
/// </summary>
public class Endpoint : EndpointWithoutRequest<Response>
{
    /// <summary>
    /// Configures the endpoint for sysinfo.
    /// </summary>
    public override void Configure()
    {
        Get("/sysinfo");
        AllowAnonymous();
    }

    /// <summary>
    /// Handles the asynchronous execution of the <see cref="Endpoint"/> request.
    /// </summary>
    /// <param name="ct">The cancellation token.</param>
    /// <returns>A task representing the asynchronous operation.</returns>
    public override async Task HandleAsync(CancellationToken ct)
    {
        await SendAsync(new Response
        {
            Author = "jha",
            Language = ".NET 8 / C# 12.0",
            Framework = "FastEndpoints",
            Multithreaded = true,
            Comment = "This is a comment",
            Version = "v1.0.0"
        }, cancellation: ct);
    }
}