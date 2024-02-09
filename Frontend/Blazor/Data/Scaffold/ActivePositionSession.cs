namespace Blazor.Data.Scaffold;

public class ActivePositionSession
{
    public Guid Id { get; set; }

    public DateTime StartTime { get; set; }

    public DateTime? EndTime { get; set; }

    public DateTime LastUpdated { get; set; }

    public TimeSpan Duration { get; set; }

    public DateTime DatafeedFirst { get; set; }

    public DateTime DatafeedLast { get; set; }

    public bool IsActive { get; set; }

    public string? AssocVnasFacilities { get; set; }

    public string PositionSimpleCallsign { get; set; } = null!;

    public virtual ICollection<PositionSessionFacilityJoin> PositionSessionFacilityJoins { get; set; } =
        new List<PositionSessionFacilityJoin>();
}