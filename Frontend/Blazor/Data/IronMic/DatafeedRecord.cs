namespace Blazor.Data.IronMic;

public class DatafeedRecord
{
    public int Id { get; set; }
    public DateTime UpdateTimestamp { get; set; }
    public int NumTrackedControllerSessions { get; set; }
    public int NumTrackedPositionSessions { get; set; }
}