using BackendThingi.Api.Model;

namespace BackendThingi.Api.Features.Public.Articles.Import;

/// <summary>
/// Represents a request for importing articles.
/// </summary>
public class Request
{
    /// <summary>
    /// Gets or sets a value indicating whether to return the items.
    /// </summary>
    /// <remarks>
    /// This property determines whether the items should be included in the response or not.
    /// </remarks>
    public bool ReturnItems { get; set; }
}

/// <summary>
/// Represents the response of the article import operation.
/// </summary>
public class Response
{
    /// <summary>
    /// Represents the number of lines processed during the import operation.
    /// </summary>
    public int LinesProcessed { get; set; }

    /// <summary>
    /// Represents the number of database rows written during the import process.
    /// </summary>
    /// <remarks>
    /// This property is used to track the number of rows that were successfully written to the database during the import process.
    /// It is incremented each time a row is successfully written to the database.
    /// </remarks>
    /// <value>The number of database rows written.</value>
    public int DbRowsWritten { get; set; }

    /// <summary>
    /// Represents a response object for the Articles import operation.
    /// </summary>
    public List<ArticleLine> Articles { get; set; }
}