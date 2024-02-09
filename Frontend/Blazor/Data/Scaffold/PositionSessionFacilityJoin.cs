namespace Blazor.Data.Scaffold;

public class PositionSessionFacilityJoin
{
    public int Id { get; set; }

    public Guid PositionSessionId { get; set; }

    public bool PositionSessionIsActive { get; set; }

    public string FacilityId { get; set; } = null!;

    public string? FrozenData { get; set; }

    public virtual ActivePositionSession ActivePositionSession { get; set; } = null!;

    public virtual CompletedPositionSession CompletedPositionSession { get; set; } = null!;

    public virtual Facility Facility { get; set; } = null!;
}