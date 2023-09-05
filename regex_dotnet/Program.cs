using System.Text.Json;
using System.Text.Json.Serialization;

// Read the input.json file from the current directory
string json = File.ReadAllText("../commons/input.json");

// Deserialize the JSON array into a list of TextItem objects
List<Entry> entries = JsonSerializer.Deserialize<List<Entry>>(json)!;

// Print each item to the standard output
foreach (var entry in entries)
{
    Console.WriteLine(entry.Text);
}

public class Entry
{
    [JsonPropertyName("text")]
    public required string Text { get; set; }
}

