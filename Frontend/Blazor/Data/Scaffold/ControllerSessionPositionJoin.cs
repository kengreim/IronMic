namespace Blazor.Data.Scaffold;

public class ControllerSessionPositionJoin
{
    public int Id { get; set; }

    public Guid ControllerSessionId { get; set; }

    public bool ControllerSessionIsActive { get; set; }

    public string PositionId { get; set; } = null!;

    public string PositionParentFacilityId { get; set; } = null!;

    public string? FrozenData { get; set; }

    public virtual ActiveControllerSession ActiveControllerSession { get; set; } = null!;

    public virtual CompletedControllerSession CompletedControllerSession { get; set; } = null!;

    public virtual Position Position { get; set; } = null!;
}