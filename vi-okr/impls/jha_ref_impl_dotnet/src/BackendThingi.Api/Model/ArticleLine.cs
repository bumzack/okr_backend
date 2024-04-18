using BackendThingi.Api.Utils;

namespace BackendThingi.Api.Model;

/// <summary>
/// Represents an article with layout specifications for each property.
/// </summary>
public class ArticleLine
{
    /// <summary>
    /// Gets or sets the code of the article. The code is left-padded with zeros up to 20 characters.
    /// </summary>
    [Layout(0, 20, Padding.Left, '0')]
    public string? Code { get; set; }

    /// <summary>
    /// Gets or sets the description of the article. The description is right-aligned and occupies up to 1700 characters.
    /// </summary>
    [Layout(120, 1700)]
    public string? Description { get; set; }

    /// <summary>
    /// Gets or sets the title of the article. The title occupies up to 100 characters.
    /// </summary>
    [Layout(20, 100)]
    public string? Title { get; set; }

    /// <summary>
    /// Gets or sets the categories associated with the article. The categories field occupies up to 200 characters.
    /// </summary>
    [Layout(2020, 200)]
    public string? Categories { get; set; }

    /// <summary>
    /// Gets or sets additional attributes of the article. The attributes field occupies up to 200 characters.
    /// </summary>
    [Layout(1820, 200)]
    public string? Attributes { get; set; }

    /// <summary>
    /// Gets or sets the price of the article. The price is left-padded with zeros up to 20 characters.
    /// </summary>
    [Layout(2250, 20, Padding.Left, '0')]
    public float Price { get; set; }

    /// <summary>
    /// Gets or sets the positional information of the article. Leading zeros are trimmed, and the field occupies up to 30 characters.
    /// </summary>
    [Layout(2220, 30, trimLeadingZero: true)]
    public string? Pos { get; set; }

    /// <summary>
    /// Gets or sets the start date of the article's validity. The date is a unix timestamp, left-padded with zeros and occupies 25 characters.
    /// </summary>
    [Layout(2270, 25, Padding.Left, '0')]
    public DateTime StartDate { get; set; }

    /// <summary>
    /// Gets or sets the end date of the article's validity. The date is a unix timestamp, left-padded with zeros and occupies 25 characters.
    /// </summary>
    [Layout(2295, 25, Padding.Left, '0')]
    public DateTime EndDate { get; set; }
}