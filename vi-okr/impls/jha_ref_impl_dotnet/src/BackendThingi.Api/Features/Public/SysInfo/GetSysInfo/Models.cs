namespace BackendThingi.Api.Features.Public.SysInfo.GetSysInfo;

/// <summary>
/// Represents the response model for retrieving system information.
/// </summary>
public class Response
{
    /// <summary>
    /// The author of the system information.
    /// </summary>
    public string? Author { get; set; }

    /// <summary>
    /// Represents the language used in the system.
    /// </summary>
    public string? Language { get; set; }

    /// <summary>
    /// Represents the framework used by the system.
    /// </summary>
    public string? Framework { get; set; }

    /// <summary>
    /// Represents a property indicating whether the operation is multithreaded or not.
    /// </summary>
    /// <value>
    /// <c>true</c> if the operation is multithreaded; otherwise, <c>false</c>.
    /// </value>
    public bool? Multithreaded { get; set; }

    /// <summary>
    /// Represents a comment property for system information.
    /// </summary>
    public string? Comment { get; set; }

    /// <summary>
    /// Represents the version of the system.
    /// </summary>
    public string? Version { get; set; }
}