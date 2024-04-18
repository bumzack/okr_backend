namespace BackendThingi.Api;

/// <summary>
/// Represents the options for the BackendThingi API.
/// </summary>
public class BackendThingiOptions
{
    /// <summary>
    /// Represents the options key for the BackendThingi API.
    /// </summary>
    public const string BackendThingi = "BackendThingi";

    /// <summary>
    /// Represents the path of the directory where the article files are located.
    /// </summary>
    /// <remarks>
    /// This property is used to specify the directory where the article files
    /// are stored. It is used by the <see cref="ArticleService"/> class to import
    /// articles from the files in this directory.
    /// </remarks>
    public string? ArticlesFilesDirPath { get; init; }
}