namespace Blazor.Data.Scaffold;

public class Artcc
{
    public string Id { get; set; } = null!;

    public DateTime LastUpdated { get; set; }

    public virtual ICollection<Facility> Facilities { get; set; } = new List<Facility>();
}