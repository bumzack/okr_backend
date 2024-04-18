using System.Reflection;

namespace BackendThingi.Api.Utils;

public class FixedWidthStreamReader<T> : StreamReader where T : class
{
    private struct PropertyInfoData
    {
        public PropertyInfo PropertyInfo;
        public int Index;
        public int Length;
        public Padding Padding;
        public char PaddingChar;
        public bool TrimLeadingZero;
    }

    private int _lineLength;
    private readonly List<PropertyInfoData> _propertyInfoDatas = [];

    private readonly char[] _lineBuffer;

    public FixedWidthStreamReader(Stream stream) : base(stream)
    {
        InitPropertySetters();
        _lineBuffer = new char[_lineLength + 1];
    }

    public FixedWidthStreamReader(string file) : base(file)
    {
        InitPropertySetters();
        _lineBuffer = new char[_lineLength + 1];
    }

    private void InitPropertySetters()
    {
        var type = typeof(T);

        foreach (var propertyInfo in type.GetProperties())
        {
            var layoutAttribute = propertyInfo.GetCustomAttribute<LayoutAttribute>();
            if (layoutAttribute != null)
            {
                _propertyInfoDatas.Add(new PropertyInfoData
                {
                    PropertyInfo = propertyInfo,
                    Index = layoutAttribute.Index,
                    Length = layoutAttribute.Length,
                    Padding = layoutAttribute.Padding,
                    PaddingChar = layoutAttribute.PaddingChar,
                    TrimLeadingZero = layoutAttribute.TrimLeadingZero
                });

                _lineLength += layoutAttribute.Length;
            }
        }
    }

    public new T? ReadLine()
    {
        if (Peek() < 0)
        {
            return null;
        }

        var lineBuffer = _lineBuffer.AsSpan();
        ReadBlock(lineBuffer);
        return ReadT(lineBuffer);
    }

    private T? ReadT(ReadOnlySpan<char> line)
    {
        if (line.Length != _lineLength + 1)
        {
            return null;
        }

        var obj = Activator.CreateInstance<T>();
        foreach (var propertyInfoData in _propertyInfoDatas)
        {
            if (line.Length <= propertyInfoData.Index) continue;

            var value = ExtractValue(line[.._lineLength], propertyInfoData);
            SetValue(obj, propertyInfoData.PropertyInfo, value);
        }

        return obj;
    }

    private ReadOnlySpan<char> ExtractValue(ReadOnlySpan<char> line, PropertyInfoData propertyInfoData)
    {
        var value = line.Slice(propertyInfoData.Index, propertyInfoData.Length);
        if (propertyInfoData.Padding == Padding.Left)
        {
            value = TrimStart(value, propertyInfoData.PaddingChar);
        }
        else
        {
            value = TrimEnd(value, propertyInfoData.PaddingChar);
        }

        if (propertyInfoData.TrimLeadingZero)
        {
            value = TrimStart(value, '0');
        }

        return value;
    }

    private ReadOnlySpan<char> TrimStart(ReadOnlySpan<char> span, char trimChar)
    {
        var start = 0;
        for (; start < span.Length; start++)
        {
            if (span[start] != trimChar) break;
        }

        return span[start..];
    }

    private ReadOnlySpan<char> TrimEnd(ReadOnlySpan<char> span, char trimChar)
    {
        var end = span.Length - 1;
        for (; end >= 0; end--)
        {
            if (span[end] != trimChar) break;
        }

        return span[..(end + 1)];
    }

    private void SetValue(T obj, PropertyInfo info, ReadOnlySpan<char> value)
    {
        if (info.PropertyType == typeof(DateTime))
        {
            if (long.TryParse(value, out var unixTime))
            {
                info.SetValue(obj, DateTimeOffset.FromUnixTimeSeconds(unixTime).DateTime);
            }
            else
            {
                throw new FormatException($"Could not parse {value} to DateTime");
            }
        }
        else if (info.PropertyType == typeof(float))
        {
            if (float.TryParse(value, out var floatValue))
            {
                info.SetValue(obj, floatValue);
            }
            else
            {
                throw new FormatException($"Could not parse {value} to float");
            }
        }
        else
        {
            info.SetValue(obj, value.ToString());
        }
    }
}