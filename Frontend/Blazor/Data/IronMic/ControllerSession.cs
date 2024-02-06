namespace Blazor.Data.IronMic;

public class ControllerSession
{
    public Guid Id { get; set; }
    public DateTime StartTime { get; set; }
    public DateTime? EndTime { get; set; }
    public DateTime LastUpdated { get; set; }
    public TimeSpan Duration { get; set; }
    public bool IsActive { get; set; }
    public int Cid { get; set; }
    public List<AssociatedVnasPosition>? AssociatedVnasPositions { get; set; }
    public string PositionSimpleCallsign { get; set; }
    public string ConnectedCallsign { get; set; }
    public string ConnectedFrequency { get; set; }
    public Guid PositionSessionId { get; set; }
    public bool IsPositionSessionActive { get; set; }

    // Foreign Key Entity
    public PositionSession PositionSession { get; set; }
}

public class AssociatedVnasPosition
{
    public string Id { get; set; }
    public string Name { get; set; }
    public string RadioName { get; set; }
    public string Callsign { get; set; }
    public int Frequency { get; set; }
    public bool IsStarred { get; set; }
}

public class ActiveControllerSession : ControllerSession;

public class EndedControllerSession : ControllerSession;