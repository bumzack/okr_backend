namespace BackendThingi.Api.Utils;

/// <summary>
/// Attribute used to define layout specifications for properties, such as start-index, length, and padding.
/// This is used to control how property values are parsed in the fixed-width text format.
/// </summary>
[AttributeUsage(AttributeTargets.Property)]
public class LayoutAttribute(
    int index,
    int length,
    Padding padding = Padding.Right,
    char paddingChar = ' ',
    bool trimLeadingZero = false) : Attribute
{
    /// <summary>
    /// Gets the index indicating the position of the property in the layout.
    /// </summary>
    public int Index { get; private set; } = index;

    /// <summary>
    /// Gets the length of the property value in the layout.
    /// </summary>
    public int Length { get; private set; } = length;

    /// <summary>
    /// Gets the padding direction (left or right) applied to the property value if it does not fill the specified length.
    /// </summary>
    public Padding Padding { get; private set; } = padding;

    /// <summary>
    /// Gets the character used for padding the property value.
    /// </summary>
    public char PaddingChar { get; private set; } = paddingChar;

    /// <summary>
    /// Gets a value indicating whether leading zeros should be trimmed from the property value.
    /// </summary>
    public bool TrimLeadingZero { get; private set; } = trimLeadingZero;
}

/// <summary>
/// Enumerates the padding options for layout attributes.
/// </summary>
public enum Padding
{
    Left,
    Right
}