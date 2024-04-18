using BackendThingi.Api.Services;

namespace BackendThingi.Api.Features.Public.Articles.Import;

/// <summary>
/// Represents an endpoint for importing articles.
/// </summary>
public class Endpoint(ArticleService articleService) : Endpoint<Request, Response>
{
    /// <summary>
    /// Configures the endpoint for importing articles.
    /// </summary>
    public override void Configure()
    {
        Post("/articles/import");
        AllowAnonymous();
    }

    /// <summary>
    /// Handles the asynchronous request for importing articles.
    /// </summary>
    /// <param name="req">The request object containing the import settings.</param>
    /// <param name="ct">The cancellation token.</param>
    /// <returns>A task representing the asynchronous operation with the response of the import operation.</returns>
    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        //await SendAsync(await articleService.Import(true, ct), cancellation: ct);
        await SendAsync(await articleService.ImportMutli(req.ReturnItems, ct), cancellation: ct);
    }
}