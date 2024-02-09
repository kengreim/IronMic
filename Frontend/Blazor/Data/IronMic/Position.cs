namespace Blazor.Data.IronMic;

public class Position
{
    public string Id { get; set; }
    public string Name { get; set; }
    public string RadioName { get; set; }
    public string Callsign { get; set; }
    public string CallsignPrefix { get; set; }
    public string? CallsignInfix { get; set; }
    public string CallsignSuffix { get; set; }
    public string CallsignWithoutInfix { get; set; }
    public int Frequency { get; set; }
    public bool IsStarred { get; set; }
    public string ParentFacilityId { get; set; }
    public DateTime LastUpdated { get; set; }

    // Foreign Keyed Entity
    public Facility ParentFacility { get; set; }

    public List<ControllerSession> ControllerSessions { get; set; }
}