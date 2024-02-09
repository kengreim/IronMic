namespace Blazor.Data.Scaffold;

public class ActiveControllerSession
{
    public Guid Id { get; set; }

    public DateTime StartTime { get; set; }

    public DateTime? EndTime { get; set; }

    public DateTime LastUpdated { get; set; }

    public TimeSpan Duration { get; set; }

    public DateTime DatafeedFirst { get; set; }

    public DateTime DatafeedLast { get; set; }

    public bool IsActive { get; set; }

    public int Cid { get; set; }

    public string? AssocVnasPositions { get; set; }

    public string PositionSimpleCallsign { get; set; } = null!;

    public string ConnectedCallsign { get; set; } = null!;

    public string ConnectedFrequency { get; set; } = null!;

    public Guid PositionSessionId { get; set; }

    public bool PositionSessionIsActive { get; set; }

    public virtual ICollection<ControllerSessionPositionJoin> ControllerSessionPositionJoins { get; set; } =
        new List<ControllerSessionPositionJoin>();
}