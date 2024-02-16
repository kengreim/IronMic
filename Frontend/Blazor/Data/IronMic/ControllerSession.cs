namespace Blazor.Data.IronMic;

public class ControllerSession : ISession
{
    public int Cid { get; set; }
    public string ConnectedCallsign { get; set; }
    public string ConnectedFrequency { get; set; }
    public Guid PositionSessionId { get; set; }
    public bool IsPositionSessionActive { get; set; }

    // Foreign Key Entity
    public PositionSession PositionSession { get; set; }
    public Guid Id { get; set; }
    public DateTime StartTime { get; set; }
    public DateTime? EndTime { get; set; }
    public DateTime LastUpdated { get; set; }
    public TimeSpan Duration { get; set; }
    public DateTime DatafeedFirstSeen { get; set; }
    public DateTime DatafeedLastSeen { get; set; }
    public bool IsActive { get; set; }
    public string PositionSimpleCallsign { get; set; }
    public bool IsCoolingDown { get; set; }

    //public List<Position> Positions { get; set; }
}