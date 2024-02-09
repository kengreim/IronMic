namespace Blazor.Data.IronMic;

public class Facility
{
    public string Id { get; set; }
    public string Name { get; set; }
    public FacilityType Type { get; set; }
    public DateTime LastUpdated { get; set; }
    public string? ParentFacilityId { get; set; }
    public string ParentArtccId { get; set; }

    // Foreign Keyed Entities
    public Facility? ParentFacility { get; set; }
    public List<Facility>? ChidFacilities { get; set; }
    public Artcc Artcc { get; set; }
    public List<Position> Positions { get; set; }
}

public enum FacilityType
{
    Artcc,
    Tracon,
    AtctTracon,
    AtctRapcon,
    Atct
}