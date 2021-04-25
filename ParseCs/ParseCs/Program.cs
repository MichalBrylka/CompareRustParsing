using System;

using Nemesis.TextParsers;

var sum = 0L;
var iterations = long.Parse(args[0]);
for (int i = 0; i < iterations; i++)
    sum = SumLine(args[1]);

Console.WriteLine(sum);

static long SumLine(string text)
{
    var result = 0L;
    var split = text.AsSpan().Split('|');

    foreach (var v in split)
        result += long.Parse(v.Trim());

    return result;
}
