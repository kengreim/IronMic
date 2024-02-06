namespace Blazor.Data.IronMic;

public class PositionSession
{
    public Guid Id { get; set; }
    public DateTime StartTime { get; set; }
    public DateTime? EndTime { get; set; }
    public DateTime LastUpdated { get; set; }
    public TimeSpan Duration { get; set; }
    public bool IsActive { get; set; }
    public List<AssociatedVnasFacility>? AssociatedVnasFacilities { get; set; }
    public string PositionSimpleCallsign { get; set; }
}

public class AssociatedVnasFacility
{
    public string Id { get; set; }
    public string Name { get; set; }
}

public class ActivePositionSession : PositionSession;

public class EndedPositionSession : PositionSession;