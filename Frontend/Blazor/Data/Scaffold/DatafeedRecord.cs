namespace Blazor.Data.Scaffold;

public class DatafeedRecord
{
    public int Id { get; set; }

    public DateTime Update { get; set; }

    public int NumTrackedControllerSessions { get; set; }

    public int NumTrackedPositionSessions { get; set; }
}