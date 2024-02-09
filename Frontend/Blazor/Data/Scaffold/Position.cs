namespace Blazor.Data.Scaffold;

public class Position
{
    public string Id { get; set; } = null!;

    public string Name { get; set; } = null!;

    public string RadioName { get; set; } = null!;

    public string Callsign { get; set; } = null!;

    public string CallsignPrefix { get; set; } = null!;

    public string? CallsignInfix { get; set; }

    public string CallsignSuffix { get; set; } = null!;

    public string CallsignWithoutInfix { get; set; } = null!;

    public int Frequency { get; set; }

    public bool Starred { get; set; }

    public string ParentFacilityId { get; set; } = null!;

    public DateTime LastUpdated { get; set; }

    public virtual ICollection<ControllerSessionPositionJoin> ControllerSessionPositionJoins { get; set; } =
        new List<ControllerSessionPositionJoin>();

    public virtual Facility ParentFacility { get; set; } = null!;
}