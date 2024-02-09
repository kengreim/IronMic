namespace Blazor.Data.IronMic;

public class PositionSession
{
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
}