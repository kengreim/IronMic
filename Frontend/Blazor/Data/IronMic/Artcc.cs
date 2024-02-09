namespace Blazor.Data.IronMic;

public class Artcc
{
    public string Id { get; set; }
    public DateTime LastUpdated { get; set; }

    // Foreign Keyed Entity
    public List<Facility> Facilities { get; set; }
}