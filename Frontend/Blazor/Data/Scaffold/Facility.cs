namespace Blazor.Data.Scaffold;

public class Facility
{
    public string Id { get; set; } = null!;

    public string Name { get; set; } = null!;

    public string Type { get; set; } = null!;

    public DateTime LastUpdated { get; set; }

    public string? ParentFacilityId { get; set; }

    public string? ParentArtccId { get; set; }

    public virtual ICollection<Facility> InverseParentFacility { get; set; } = new List<Facility>();

    public virtual Artcc? ParentArtcc { get; set; }

    public virtual Facility? ParentFacility { get; set; }

    public virtual ICollection<PositionSessionFacilityJoin> PositionSessionFacilityJoins { get; set; } =
        new List<PositionSessionFacilityJoin>();

    public virtual ICollection<Position> Positions { get; set; } = new List<Position>();
}