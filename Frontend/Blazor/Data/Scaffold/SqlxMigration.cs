namespace Blazor.Data.Scaffold;

public class SqlxMigration
{
    public long Version { get; set; }

    public string Description { get; set; } = null!;

    public DateTime InstalledOn { get; set; }

    public bool Success { get; set; }

    public byte[] Checksum { get; set; } = null!;

    public long ExecutionTime { get; set; }
}