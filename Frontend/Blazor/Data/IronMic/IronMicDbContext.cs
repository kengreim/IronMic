using Microsoft.EntityFrameworkCore;

namespace Blazor.Data.IronMic;

public class IronMicDbContext : DbContext
{
    public required DbSet<Artcc> Artccs { get; init; }
    public required DbSet<Facility> Facilities { get; init; }
    public required DbSet<Position> Positions { get; set; }
    public required DbSet<ControllerSession> ControllerSessions { get; init; }
    public required DbSet<PositionSession> PositionSessions { get; init; }
}