namespace Blazor.Data.IronMic;

public class ControllerSessionPosition
{
    public Guid ControllerSessionId { get; set; }
    public bool ControllerSessionIsActive { get; set; }
    public string PositionId { get; set; }
    public string PositionParentFacilityId { get; set; }
    public PositionInfo PostionData { get; set; }
}

public class PositionInfo
{
    public string Id { get; set; }
    public string Name { get; set; }
    public string RadioName { get; set; }
    public string Callsign { get; set; }
    public int Frequency { get; set; }
    public bool IsStarred { get; set; }
    public string ParentFacilityId { get; set; }
}